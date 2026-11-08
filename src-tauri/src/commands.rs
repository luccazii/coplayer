use crate::storage;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachmentPolicy {
    max_attachments: usize,
    max_image_bytes: u64,
    max_total_bytes: u64,
    max_dimension: u32,
}

#[tauri::command]
pub fn get_attachment_policy() -> AttachmentPolicy {
    AttachmentPolicy {
        max_attachments: storage::MAX_ATTACHMENTS,
        max_image_bytes: storage::MAX_IMAGE_BYTES,
        max_total_bytes: storage::MAX_TOTAL_BYTES,
        max_dimension: storage::MAX_DIMENSION,
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    name: &'static str,
    version: &'static str,
    platform: &'static str,
    architecture: &'static str,
}

#[tauri::command]
pub fn get_app_info() -> AppInfo {
    AppInfo {
        name: "Coplayer",
        version: env!("CARGO_PKG_VERSION"),
        platform: std::env::consts::OS,
        architecture: std::env::consts::ARCH,
    }
}

use crate::campaign::{NewCampaign, Workspace};
use storage::StorageState;
use tauri::State;

#[tauri::command]
pub fn campaign_workspace(state: State<'_, StorageState>) -> Result<Workspace, String> {
    state.0.lock().map_err(|e| e.to_string())?.workspace()
}
#[tauri::command]
pub fn campaign_create(
    input: NewCampaign,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .create_campaign(input)
}
#[tauri::command]
pub fn campaign_select(
    campaign_id: String,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .select_campaign(&campaign_id)
}
#[tauri::command]
pub fn chat_create(
    campaign_id: String,
    title: String,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .create_chat(&campaign_id, &title)
}
#[tauri::command]
pub fn chat_select(
    campaign_id: String,
    chat_id: String,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .select_chat(&campaign_id, &chat_id)
}
#[tauri::command]
pub fn chat_save_draft(
    campaign_id: String,
    chat_id: String,
    revision: i64,
    draft: String,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .save_draft(&campaign_id, &chat_id, revision, &draft)
}

#[tauri::command]
pub fn chat_remove_attachment(
    target: crate::campaign::DraftTarget,
    id: String,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .remove_chat_attachment(&target, &id)
}
#[tauri::command]
pub fn chat_submit_message(
    input: crate::campaign::SubmitMessage,
    state: State<'_, StorageState>,
) -> Result<Workspace, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .submit_message(input)
}

#[tauri::command]
pub fn chat_message_attachments(
    campaign_id: String,
    chat_id: String,
    message_id: String,
    state: State<'_, StorageState>,
) -> Result<Vec<storage::Attachment>, String> {
    state
        .0
        .lock()
        .map_err(|e| e.to_string())?
        .message_attachments(&campaign_id, &chat_id, &message_id)
}
