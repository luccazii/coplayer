import { invoke } from "@tauri-apps/api/core";

export interface ModelOption { id: string; name: string; isDefault: boolean }
export type { Attachment as ProofImage } from "./attachments";
import type { Attachment } from "./attachments";
export interface ProofAnalysis {
  requestId: string;
  threadId: string | null;
  turnId: string | null;
  status: "starting" | "running" | "cancelling" | "completed" | "cancelled" | "failed";
  text: string;
  message: string;
  model: string;
  imageCount: number;
  cancelRequested: boolean;
}
export interface ProviderStatus {
  runtime: "stopped" | "starting" | "ready" | "failed";
  runtimeVersion: string;
  authenticated: boolean;
  plan: string | null;
  loginPending: boolean;
  loginUrl: string | null;
  message: string;
  models: ModelOption[];
  analysis: ProofAnalysis | null;
}
export const providerStatus = () => invoke<ProviderStatus>("provider_status");
export const providerStart = () => invoke<ProviderStatus>("provider_start");
export const providerLogin = () => invoke<void>("provider_login");
export const providerCancelLogin = () => invoke<void>("provider_cancel_login");
export const providerImportImages = () => invoke<Attachment[]>("provider_import_images");
export const providerRemoveImage = (id: string) => invoke<void>("provider_remove_image", { id });
export const providerAnalyze = (prompt: string, model: string, imageIds: string[]) =>
  invoke<void>("provider_analyze", { prompt, model, imageIds });
export const providerCancel = () => invoke<void>("provider_cancel");
export const providerStop = () => invoke<void>("provider_stop");

export const providerImages = () => invoke<Attachment[]>("provider_images");
