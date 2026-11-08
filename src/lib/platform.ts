import { invoke } from "@tauri-apps/api/core";

export interface WindowTarget {
  id: number;
  pid: number;
  title: string;
  appName: string;
  width: number;
  height: number;
}

import type { Attachment } from "./attachments";
export type CaptureArtifact = Attachment;

export interface PlatformStatus {
  platform: string;
  supported: boolean;
  permissionGranted: boolean;
  selected: WindowTarget | null;
  captures: CaptureArtifact[];
  busy: boolean;
  iconVisible: boolean;
  shortcutError: string | null;
}

export const platformStatus = () => invoke<PlatformStatus>("platform_status");
export const requestScreenPermission = () => invoke<void>("platform_request_permission");
export const listCaptureWindows = () => invoke<WindowTarget[]>("platform_list_windows");
export const selectCaptureWindow = (target: WindowTarget) =>
  invoke<WindowTarget>("platform_select_window", { windowId: target.id, pid: target.pid });
export const captureWindow = () => invoke<CaptureArtifact>("platform_capture");
export const togglePanel = () => invoke<void>("platform_toggle_panel");
export const dragIcon = () => invoke<void>("platform_drag_icon");
export const setIconVisible = (visible: boolean) => invoke<void>("platform_set_icon_visible", { visible });
export const removeCapture = (id: string) => invoke<void>("platform_remove_capture", { id });
