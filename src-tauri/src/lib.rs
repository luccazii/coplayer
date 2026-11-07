mod commands;
mod platform;
mod provider;
mod storage;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .manage(platform::PlatformState::default())
        .manage(provider::ProviderState::default())
        .setup(|app| {
            let root = app.path().app_local_data_dir()?.join("data-v1");
            app.manage(storage::StorageState(std::sync::Mutex::new(
                storage::Store::open(&root)?,
            )));
            platform::setup(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() == "main"
                && let tauri::WindowEvent::CloseRequested { api, .. } = event
            {
                if platform::close_to_overlay(window.app_handle()) {
                    api.prevent_close();
                } else {
                    window.app_handle().exit(0);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_app_info,
            commands::get_attachment_policy,
            platform::platform_status,
            platform::platform_request_permission,
            platform::platform_list_windows,
            platform::platform_select_window,
            platform::platform_capture,
            platform::platform_toggle_panel,
            platform::platform_drag_icon,
            platform::platform_set_icon_visible,
            platform::platform_remove_capture,
            provider::provider_status,
            provider::provider_images,
            provider::provider_start,
            provider::provider_login,
            provider::provider_cancel_login,
            provider::provider_import_images,
            provider::provider_remove_image,
            provider::provider_analyze,
            provider::provider_cancel,
            provider::provider_stop,
        ])
        .build(tauri::generate_context!())
        .expect("failed to build Coplayer")
        .run(|app, event| {
            if let tauri::RunEvent::Exit = event {
                tauri::async_runtime::block_on(app.state::<provider::ProviderState>().shutdown());
            }
        });
}
