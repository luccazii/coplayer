use crate::storage::{self, Attachment, PendingAttachment, Source, StorageState};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowTarget {
    pub id: u32,
    pub pid: i32,
    pub title: String,
    pub app_name: String,
    pub width: f64,
    pub height: f64,
}

pub type CaptureArtifact = Attachment;

#[derive(Default)]
pub struct PlatformState {
    selected: Mutex<Option<WindowTarget>>,
    busy: AtomicBool,
    shortcut_error: Mutex<Option<String>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformStatus {
    supported: bool,
    permission_granted: bool,
    selected: Option<WindowTarget>,
    captures: Vec<CaptureArtifact>,
    busy: bool,
    icon_visible: bool,
    shortcut_error: Option<String>,
}

struct OperationGuard<'a>(&'a AtomicBool);
impl Drop for OperationGuard<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

fn begin(state: &PlatformState) -> Result<OperationGuard<'_>, String> {
    state
        .busy
        .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
        .map_err(|_| "Aguarde a operação de captura ou foco em andamento.".to_string())?;
    Ok(OperationGuard(&state.busy))
}

#[cfg(target_os = "macos")]
fn native(request: Value) -> Result<Value, String> {
    use std::ffi::{CStr, CString, c_char};
    unsafe extern "C" {
        fn coplayer_platform_request(input: *const c_char) -> *mut c_char;
        fn coplayer_platform_free(value: *mut c_char);
    }
    let input = CString::new(request.to_string()).map_err(|e| e.to_string())?;
    // The bridge returns an owned, NUL-terminated UTF-8 JSON allocation.
    let output = unsafe { coplayer_platform_request(input.as_ptr()) };
    if output.is_null() {
        return Err("O bridge nativo não retornou resultado.".into());
    }
    let encoded = unsafe { CStr::from_ptr(output) }
        .to_string_lossy()
        .into_owned();
    unsafe { coplayer_platform_free(output) };
    let value: Value = serde_json::from_str(&encoded).map_err(|e| e.to_string())?;
    if let Some(error) = value.get("error").and_then(Value::as_str) {
        return Err(error.to_owned());
    }
    Ok(value)
}

#[cfg(not(target_os = "macos"))]
fn native(request: Value) -> Result<Value, String> {
    if request["action"] == "status" {
        Ok(json!({"supported": false, "permissionGranted": false}))
    } else {
        Err("A prova de plataforma 0.3 está disponível somente no macOS.".into())
    }
}

async fn worker<T: Send + 'static>(
    operation: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String> {
    tauri::async_runtime::spawn_blocking(operation)
        .await
        .map_err(|e| e.to_string())?
}

fn targets() -> Result<Vec<WindowTarget>, String> {
    serde_json::from_value(native(json!({"action": "windows"}))?["windows"].clone())
        .map_err(|e| e.to_string())
}

fn selection(app: &AppHandle) -> Result<WindowTarget, String> {
    app.state::<PlatformState>()
        .selected
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "Selecione uma janela antes de capturar ou devolver o foco.".into())
}

fn toggle_panel(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<PlatformState>();
    let _guard = begin(&state)?;
    let panel = app
        .get_webview_window("main")
        .ok_or("Painel indisponível.")?;
    if panel.is_visible().map_err(|e| e.to_string())? {
        let target = selection(app)?;
        panel.hide().map_err(|e| e.to_string())?;
        if let Err(error) = native(json!({"action": "activate", "pid": target.pid})) {
            let _ = panel.show();
            let _ = panel.set_focus();
            return Err(error);
        }
    } else {
        panel.show().map_err(|e| e.to_string())?;
        panel.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn capture(app: &AppHandle) -> Result<CaptureArtifact, String> {
    let state = app.state::<PlatformState>();
    let _guard = begin(&state)?;
    let selected = selection(app)?;
    let directory = app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("data-v1/staging");
    storage::private_directory(&directory)?;
    let path = directory.join(format!("{}.png", storage::identifier()));
    let windows = app.webview_windows();
    let mut hidden: Vec<String> = Vec::new();
    for window in windows.values() {
        let visible = match window.is_visible() {
            Ok(visible) => visible,
            Err(error) => {
                for label in &hidden {
                    if let Some(previous) = app.get_webview_window(label) {
                        let _ = previous.show();
                    }
                }
                return Err(error.to_string());
            }
        };
        if visible {
            if let Err(error) = window.hide() {
                for label in &hidden {
                    if let Some(previous) = app.get_webview_window(label) {
                        let _ = previous.show();
                    }
                }
                return Err(error.to_string());
            }
            hidden.push(window.label().to_owned());
        }
    }
    // Allow the compositor to remove Coplayer surfaces. The ScreenCaptureKit
    // filter independently isolates the chosen window even if another app overlaps it.
    std::thread::sleep(Duration::from_millis(180));
    let result = native(
        json!({"action": "capture", "windowId": selected.id, "pid": selected.pid, "path": path, "maxDimension": storage::MAX_DIMENSION, "maxBytes": storage::MAX_IMAGE_BYTES}),
    );
    let mut restoration_error = None;
    for label in hidden {
        if let Some(window) = app.get_webview_window(&label)
            && let Err(error) = window.show()
        {
            restoration_error = Some(error.to_string());
        }
    }
    // Both button and shortcut return to the panel so the new attachment is visible.
    if let Some(panel) = app.get_webview_window("main")
        && let Err(error) = panel.show().and_then(|_| panel.set_focus())
    {
        restoration_error = Some(error.to_string());
    }
    let persisted = (|| {
        result?;
        if let Some(error) = restoration_error {
            return Err(format!(
                "Captura realizada, mas não foi possível restaurar o painel: {error}"
            ));
        }
        let bytes = storage::read_image(&path)?;
        let mut artifacts = app
            .state::<StorageState>()
            .0
            .lock()
            .map_err(|e| e.to_string())?
            .add(
                Source::Capture,
                vec![PendingAttachment {
                    name: format!("{} · {}", selected.app_name, selected.title),
                    data: bytes,
                }],
            )?;
        artifacts
            .pop()
            .ok_or_else(|| "Nenhuma captura foi armazenada.".to_string())
    })();
    let _ = std::fs::remove_file(&path);
    let artifact = persisted?;
    app.emit_to("main", "platform-capture", &artifact)
        .map_err(|e| e.to_string())?;
    Ok(artifact)
}

#[tauri::command]
pub async fn platform_status(app: AppHandle) -> Result<PlatformStatus, String> {
    worker(move || {
        let status = native(json!({"action": "status"}))?;
        let state = app.state::<PlatformState>();
        Ok(PlatformStatus {
            supported: status["supported"].as_bool().unwrap_or(false),
            permission_granted: status["permissionGranted"].as_bool().unwrap_or(false),
            selected: state.selected.lock().map_err(|e| e.to_string())?.clone(),
            captures: app
                .state::<StorageState>()
                .0
                .lock()
                .map_err(|e| e.to_string())?
                .list(Source::Capture)?,
            busy: state.busy.load(Ordering::Acquire),
            icon_visible: app
                .get_webview_window("coplayer-icon")
                .is_some_and(|window| window.is_visible().unwrap_or(false)),
            shortcut_error: state
                .shortcut_error
                .lock()
                .map_err(|e| e.to_string())?
                .clone(),
        })
    })
    .await
}

#[tauri::command]
pub async fn platform_request_permission() -> Result<(), String> {
    worker(|| native(json!({"action": "permission"})).map(|_| ())).await
}

#[tauri::command]
pub async fn platform_list_windows() -> Result<Vec<WindowTarget>, String> {
    worker(targets).await
}

#[tauri::command]
pub async fn platform_select_window(
    app: AppHandle,
    window_id: u32,
    pid: i32,
) -> Result<WindowTarget, String> {
    worker(move || {
        let state = app.state::<PlatformState>();
        let _guard = begin(&state)?;
        let selected = targets()?
            .into_iter()
            .find(|window| window.id == window_id && window.pid == pid)
            .ok_or("A janela não está mais disponível. Atualize a lista.")?;
        *state.selected.lock().map_err(|e| e.to_string())? = Some(selected.clone());
        Ok(selected)
    })
    .await
}

#[tauri::command]
pub async fn platform_capture(app: AppHandle) -> Result<CaptureArtifact, String> {
    worker(move || capture(&app)).await
}

#[tauri::command]
pub async fn platform_toggle_panel(app: AppHandle) -> Result<(), String> {
    worker(move || toggle_panel(&app)).await
}

#[tauri::command]
pub fn platform_drag_icon(app: AppHandle) -> Result<(), String> {
    app.get_webview_window("coplayer-icon")
        .ok_or("Ícone indisponível.")?
        .start_dragging()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn platform_set_icon_visible(app: AppHandle, visible: bool) -> Result<(), String> {
    let state = app.state::<PlatformState>();
    let _guard = begin(&state)?;
    if visible {
        selection(&app)?;
    }
    let icon = app
        .get_webview_window("coplayer-icon")
        .ok_or("Ícone disponível apenas no macOS.")?;
    if visible { icon.show() } else { icon.hide() }.map_err(|e| e.to_string())?;
    if let Some(panel) = app.get_webview_window("main") {
        panel
            .set_always_on_top(visible)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn platform_remove_capture(app: AppHandle, id: String) -> Result<(), String> {
    let state = app.state::<PlatformState>();
    let _guard = begin(&state)?;
    app.state::<StorageState>()
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .remove(Source::Capture, &id)
}

pub fn setup(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        use tauri_plugin_global_shortcut::{
            Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
        };
        let panel_shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Space);
        let capture_shortcut =
            Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::Digit9);
        app.plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    let app = app.clone();
                    let is_capture = shortcut == &capture_shortcut;
                    if !is_capture && shortcut != &panel_shortcut {
                        return;
                    }
                    tauri::async_runtime::spawn_blocking(move || {
                        let result = if is_capture {
                            capture(&app).map(|_| ())
                        } else {
                            toggle_panel(&app)
                        };
                        if let Err(error) = result {
                            let _ = app.emit_to("main", "platform-error", &error);
                            if !app.state::<PlatformState>().busy.load(Ordering::Acquire)
                                && let Some(panel) = app.get_webview_window("main")
                            {
                                let _ = panel.show();
                                let _ = panel.set_focus();
                            }
                        }
                    });
                })
                .build(),
        )?;
        let mut errors = Vec::new();
        for (shortcut, label) in [(panel_shortcut, "⌘⇧Espaço"), (capture_shortcut, "⌘⇧9")]
        {
            if let Err(error) = app.global_shortcut().register(shortcut) {
                errors.push(format!("{label}: {error}"));
            }
        }
        if !errors.is_empty() {
            *app.state::<PlatformState>()
                .shortcut_error
                .lock()
                .map_err(|e| e.to_string())? = Some(errors.join("; "));
        }
        tauri::WebviewWindowBuilder::new(
            app,
            "coplayer-icon",
            tauri::WebviewUrl::App("index.html?surface=icon".into()),
        )
        .title("Coplayer · Ícone")
        .inner_size(60.0, 92.0)
        .position(24.0, 140.0)
        .decorations(false)
        .resizable(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .focused(false)
        .build()?;
    }
    #[cfg(not(target_os = "macos"))]
    let _ = app;
    Ok(())
}

/// Returns true when the main-window close button must hide the panel instead
/// of destroying it. The caller must prevent the default close in that case.
pub fn close_to_overlay(app: &AppHandle) -> bool {
    let visible = app
        .get_webview_window("coplayer-icon")
        .is_some_and(|window| window.is_visible().unwrap_or(false));
    if !visible {
        return false;
    }
    let app = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        if let Err(error) = toggle_panel(&app) {
            let _ = app.emit_to("main", "platform-error", error);
        }
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_and_focus_operations_are_exclusive_and_release_on_error() {
        let state = PlatformState::default();
        let first = begin(&state).unwrap();
        assert!(begin(&state).is_err());
        drop(first);
        assert!(begin(&state).is_ok());
    }
}
