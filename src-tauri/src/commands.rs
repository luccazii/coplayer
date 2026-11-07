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
