import { invoke } from "@tauri-apps/api/core";

// Shared IPC contract: filesystem paths never cross into the webview.
export interface Attachment {
  id: string;
  name: string;
  mime: "image/png" | "image/jpeg";
  bytes: number;
  width: number;
  height: number;
  createdAtMs: number;
  available: boolean;
  previewDataUrl: string;
}
export interface AttachmentPolicy {
  maxAttachments: number;
  maxImageBytes: number;
  maxTotalBytes: number;
  maxDimension: number;
}
export const attachmentPolicy = () => invoke<AttachmentPolicy>("get_attachment_policy");
export const mebibytes = (bytes: number) => bytes / (1024 * 1024);
