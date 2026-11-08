import type { Attachment } from "./attachments";
import { invoke } from "@tauri-apps/api/core";
export interface Campaign { id: string; name: string; createdAtMs: number }
export interface Configuration {
  id: string; campaignId: string; gameVersion: string; activeDlcs: string[];
  dlcsConfirmed: boolean; noDlcsConfirmed: boolean; source: "player"; createdAtMs: number;
}
export interface Chat {
  id: string; campaignId: string; configurationId: string; title: string;
  draft: string; revision: number; draftGeneration: number; createdAtMs: number;
}
export interface Workspace {
  campaigns: Campaign[]; campaignId: string | null; chatId: string | null;
  chats: Chat[]; configuration: Configuration | null; attachments: Attachment[]; messages: LocalMessage[];
}
export interface NewCampaign {
  name: string; gameVersion: string; activeDlcs: string[];
  dlcsConfirmed: boolean; noDlcsConfirmed: boolean;
}
export const campaignWorkspace = () => invoke<Workspace>("campaign_workspace");
export const campaignCreate = (input: NewCampaign) => invoke<Workspace>("campaign_create", { input });
export const campaignSelect = (campaignId: string) => invoke<Workspace>("campaign_select", { campaignId });
export const chatCreate = (campaignId: string, title: string) => invoke<Workspace>("chat_create", { campaignId, title });
export const chatSelect = (campaignId: string, chatId: string) => invoke<Workspace>("chat_select", { campaignId, chatId });
export const chatSaveDraft = (campaignId: string, chatId: string, revision: number, draft: string) =>
  invoke<Workspace>("chat_save_draft", { campaignId, chatId, revision, draft });

export interface DraftTarget { campaignId: string; chatId: string; generation: number }
export interface LocalMessage {
  id: string; campaignId: string; chatId: string; configurationId: string;
  text: string; status: "local_only"; createdAtMs: number; attachmentCount: number;
}
export interface ChatCaptureEvent { target: DraftTarget; attachment: Attachment }
export const chatCapture = (target: DraftTarget) => invoke<Attachment>("chat_capture", { target });
export const chatRemoveAttachment = (target: DraftTarget, id: string) => invoke<Workspace>("chat_remove_attachment", { target, id });
export const chatSubmitMessage = (target: DraftTarget, revision: number, text: string, attachmentIds: string[]) =>
  invoke<Workspace>("chat_submit_message", { input: { target, revision, text, attachmentIds } });
export const chatMessageAttachments = (campaignId: string, chatId: string, messageId: string) =>
  invoke<Attachment[]>("chat_message_attachments", { campaignId, chatId, messageId });
