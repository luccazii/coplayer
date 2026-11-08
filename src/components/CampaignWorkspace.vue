<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref } from "vue";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { attachmentPolicy, mebibytes, type Attachment, type AttachmentPolicy } from "../lib/attachments";
import ChatCaptureControls from "./ChatCaptureControls.vue";
import { chatCapture, chatRemoveAttachment, chatSubmitMessage, chatMessageAttachments, type DraftTarget, type ChatCaptureEvent, campaignCreate, campaignSelect, campaignWorkspace, chatCreate, chatSaveDraft, chatSelect, type Workspace } from "../lib/campaigns";

const workspace = ref<Workspace | null>(null);
const busy = ref(false);
const error = ref("");
const notice = ref("");
const creating = ref(false);
const name = ref("");
const version = ref("");
const dlcs = ref<{ name: string; active: boolean }[]>([]);
const dlcName = ref("");
const confirmed = ref(false);
const noDlcs = ref(false);
const chatTitle = ref("");
const draft = ref("");
const historyImages = ref<Attachment[]>([]);
const historyMessageId = ref("");
const preview = ref<Attachment | null>(null);
const previewDialog = ref<HTMLDialogElement>();
const policy = ref<AttachmentPolicy | null>(null);
async function openPreview(image: Attachment) {
  preview.value = image; await nextTick(); previewDialog.value?.showModal();
}
function closePreview() { previewDialog.value?.close(); preview.value = null; }
const subscriptions: UnlistenFn[] = [];
let disposed = false;
const target = computed<DraftTarget | null>(() => active.value ? { campaignId: active.value.campaignId, chatId: active.value.id, generation: active.value.draftGeneration } : null);
const active = computed(() => workspace.value?.chats.find(chat => chat.id === workspace.value?.chatId));
const campaign = computed(() => workspace.value?.campaigns.find(item => item.id === workspace.value?.campaignId));
const activeDlcs = computed(() => dlcs.value.filter(item => item.active).map(item => item.name));
const dirty = computed(() => !!active.value && active.value.draft !== draft.value);
function apply(value: Workspace, preserveDraft = false) {
  const previousDraft = draft.value;
  const sameDraft = value.chatId === workspace.value?.chatId && value.chats.find(chat => chat.id === value.chatId)?.draftGeneration === active.value?.draftGeneration;
  historyImages.value = []; historyMessageId.value = ""; closePreview();
  workspace.value = value;
  draft.value = preserveDraft && sameDraft ? previousDraft : value.chats.find(chat => chat.id === value.chatId)?.draft ?? "";
}
async function run(action: () => Promise<void>) {
  if (busy.value) return;
  busy.value = true; error.value = ""; notice.value = "";
  try { await action(); }
  catch (cause) { error.value = String(cause); }
  finally { busy.value = false; }
}
async function save() {
  if (!active.value || !workspace.value?.campaignId || !dirty.value) return;
  apply(await chatSaveDraft(workspace.value.campaignId, active.value.id, active.value.revision, draft.value));
}
async function switchCampaign(id: string) {
  await save(); apply(await campaignSelect(id)); creating.value = false;
}
async function switchChat(id: string) {
  if (!workspace.value?.campaignId) return;
  await save(); apply(await chatSelect(workspace.value.campaignId, id));
}
function addDlc() {
  if (!dlcName.value.trim()) return;
  dlcs.value.push({ name: dlcName.value.trim(), active: false });
  dlcName.value = ""; confirmed.value = false; noDlcs.value = false;
}
async function createCampaign() {
  await save();
  apply(await campaignCreate({ name: name.value, gameVersion: version.value, activeDlcs: activeDlcs.value,
    dlcsConfirmed: confirmed.value, noDlcsConfirmed: noDlcs.value }));
  creating.value = false; name.value = ""; version.value = ""; dlcs.value = [];
  confirmed.value = false; noDlcs.value = false; dlcName.value = "";
  notice.value = "Campanha criada neste dispositivo. Nenhuma análise foi iniciada.";
}
async function createChat() {
  if (!workspace.value?.campaignId) return;
  await save(); apply(await chatCreate(workspace.value.campaignId, chatTitle.value)); chatTitle.value = ""; creating.value = false;
}
function rememberCapture(event: ChatCaptureEvent) {
  if (target.value?.chatId === event.target.chatId && target.value.campaignId === event.target.campaignId && target.value.generation === event.target.generation && workspace.value) {
    if (!workspace.value.attachments.some(item => item.id === event.attachment.id)) workspace.value.attachments.push(event.attachment);
    notice.value = "Captura adicionada ao rascunho. Nenhuma análise foi iniciada.";
  } else {
    notice.value = "Captura salva no chat de origem. A conversa atual não foi alterada.";
  }
}
async function capture() {
  await save();
  if (!target.value) return;
  const destination = { ...target.value };
  rememberCapture({ target: destination, attachment: await chatCapture(destination) });
}
async function removeAttachment(id: string) {
  if (!target.value) return;
  apply(await chatRemoveAttachment(target.value, id), true);
  notice.value = "Anexo removido do rascunho e do armazenamento local.";
}
async function submit() {
  if (!target.value || !active.value || !workspace.value) return;
  apply(await chatSubmitMessage(target.value, active.value.revision, draft.value, workspace.value.attachments.map(item => item.id)));
  notice.value = "Mensagem registrada localmente. Ainda não enviada à IA; a análise será integrada na próxima etapa.";
}
async function showHistoryImages(messageId: string) {
  if (!target.value) return;
  historyImages.value = await chatMessageAttachments(target.value.campaignId, target.value.chatId, messageId);
  historyMessageId.value = messageId;
}
onMounted(() => run(async () => {
  const webview = getCurrentWebviewWindow();
  for (const stop of [
    await webview.listen<ChatCaptureEvent>("chat-capture", event => rememberCapture(event.payload)),
    await webview.listen<string>("chat-capture-error", event => { error.value = event.payload; }),
  ]) { if (disposed) stop(); else subscriptions.push(stop); }
  if (disposed) return;
  policy.value = await attachmentPolicy();
  apply(await campaignWorkspace()); creating.value = !workspace.value?.campaigns.length;
}));
onUnmounted(() => { disposed = true; subscriptions.forEach(stop => stop()); });
</script>

<template>
  <section aria-label="Campanhas e chats locais">
    <p v-if="error" role="alert" class="notice">{{ error }}</p>
    <p v-if="notice" role="status">{{ notice }}</p>
    <p v-if="!workspace && !error" role="status">Carregando campanhas locais…</p>
    <button v-if="!workspace && error" :disabled="busy" @click="run(async () => { apply(await campaignWorkspace()); creating = !workspace?.campaigns.length; })">Tentar novamente</button>
    <div v-if="workspace" class="workspace-grid" :aria-busy="busy">
      <aside class="workspace-panel sessions">
        <label for="campaign-select">Campanha</label>
        <select id="campaign-select" :value="workspace.campaignId ?? ''" :disabled="busy || !workspace.campaigns.length" @change="run(() => switchCampaign(($event.target as HTMLSelectElement).value))">
          <option v-if="!workspace.campaigns.length" value="">Nenhuma campanha</option>
          <option v-for="item in workspace.campaigns" :key="item.id" :value="item.id">{{ item.name }}</option>
        </select>
        <button :disabled="busy" @click="creating = true">Nova campanha</button>
        <h2>Sessões de chat</h2>
        <ul class="chat-list">
          <li v-for="chat in workspace.chats" :key="chat.id"><button :disabled="busy" :aria-pressed="workspace.chatId === chat.id && !creating" @click="run(async () => { await switchChat(chat.id); creating = false; })">{{ chat.title }}</button></li>
        </ul>
        <form v-if="campaign" @submit.prevent="run(createChat)">
          <label for="chat-title">Título do novo chat</label>
          <input id="chat-title" v-model="chatTitle" :disabled="busy" maxlength="120" required />
          <button :disabled="busy || !chatTitle.trim()" type="submit">Criar chat</button>
        </form>
      </aside>
      <section class="workspace-panel conversation">
        <form v-if="creating" @submit.prevent="run(createCampaign)">
          <p class="eyebrow">VICTORIA 3</p><h2>Uma campanha, seu próprio histórico.</h2>
          <p class="muted">Informe a configuração ativa no jogo. País, data e objetivos podem ficar para depois.</p>
          <fieldset :disabled="busy">
            <label for="campaign-name">Nome da campanha</label>
            <input id="campaign-name" v-model="name" maxlength="120" required autocomplete="off" />
            <label for="game-version">Versão do jogo</label>
            <input id="game-version" v-model="version" maxlength="80" required placeholder="Digite a versão exibida no jogo" />
            <h3>Checklist de DLCs ativas</h3>
            <p class="muted">Adicione os nomes manualmente e marque somente as DLCs ativas. Ter comprado uma DLC não significa que ela está ativa. Nenhuma começa marcada.</p>
            <label for="dlc-name">Nome da DLC</label>
            <div class="dlc-entry"><input id="dlc-name" v-model="dlcName" maxlength="120" @keydown.enter.prevent="addDlc" /><button type="button" :disabled="!dlcName.trim()" @click="addDlc">Adicionar</button></div>
            <ul class="dlc-list"><li v-for="(dlc, index) in dlcs" :key="index">
              <label><input v-model="dlc.active" type="checkbox" @change="confirmed = false; noDlcs = false" /> {{ dlc.name }}</label>
              <button type="button" :aria-label="`Remover ${dlc.name}`" @click="dlcs.splice(index, 1); confirmed = false; noDlcs = false">Remover</button>
            </li></ul>
            <label class="check"><input v-model="confirmed" type="checkbox" /> Revisei e confirmo o checklist de DLCs ativas.</label>
            <label v-if="!activeDlcs.length" class="check"><input v-model="noDlcs" type="checkbox" /> Confirmo que esta campanha está sem DLCs ativas.</label>
            <div class="actions"><button type="submit" class="primary">Criar campanha</button><button v-if="campaign" type="button" @click="creating = false">Voltar ao chat</button></div>
          </fieldset>
        </form>
        <template v-else-if="active">
          <p class="eyebrow">{{ campaign?.name }}</p><h2>{{ active.title }}</h2>
          <div v-if="!workspace.messages.length" class="empty-chat"><h3>Sua conversa começa aqui.</h3><p>Reúna capturas e escreva sua pergunta. O registro é local; a análise pela assinatura será integrada na próxima etapa.</p></div>
          <ol v-else class="messages" aria-label="Mensagens locais">
            <li v-for="message in workspace.messages" :key="message.id">
              <p class="message-text">{{ message.text || 'Mensagem com capturas' }}</p>
              <p class="muted">Registrada localmente · ainda não enviada à IA</p>
              <button v-if="message.attachmentCount" type="button" :disabled="busy" @click="run(() => showHistoryImages(message.id))">Ver {{ message.attachmentCount }} anexo(s) da mensagem</button>
              <div v-if="historyMessageId === message.id" class="attachments">
                <figure v-for="image in historyImages" :key="image.id">
                  <button v-if="image.available" type="button" @click="openPreview(image)"><img :src="image.previewDataUrl" :alt="image.name" /></button>
                  <p v-else role="alert">Arquivo ausente ou alterado. Referência preservada.</p>
                  <figcaption>{{ image.name }}</figcaption>
                </figure>
              </div>
            </li>
          </ol>
          <ChatCaptureControls :disabled="busy" :at-limit="!policy || workspace.attachments.length >= policy.maxAttachments" @capture="run(capture)" />
          <p v-if="policy" class="muted">Anexos deste rascunho · {{ workspace.attachments.length }}/{{ policy.maxAttachments }} · até {{ mebibytes(policy.maxImageBytes) }} MiB por imagem e {{ mebibytes(policy.maxTotalBytes) }} MiB no total. Capturar nunca envia uma análise.</p>
          <div class="attachments" aria-label="Anexos do rascunho">
            <figure v-for="image in workspace.attachments" :key="image.id">
              <button v-if="image.available" type="button" @click="openPreview(image)"><img :src="image.previewDataUrl" :alt="image.name" /></button>
              <p v-else role="alert">Arquivo ausente ou alterado; remova antes de registrar.</p>
              <figcaption>{{ image.name }} · {{ image.width }} × {{ image.height }}</figcaption>
              <button type="button" :disabled="busy" :aria-label="`Remover anexo ${image.name}`" @click="run(() => removeAttachment(image.id))">Remover anexo</button>
            </figure>
          </div>
          <form @submit.prevent="run(async () => { await save(); notice = 'Rascunho salvo neste dispositivo.'; })">
            <label for="chat-draft">Rascunho local</label>
            <textarea id="chat-draft" v-model="draft" rows="6" maxlength="16000" :disabled="busy" placeholder="O que você quer entender sobre a partida?"></textarea>
            <div class="actions"><button type="submit" :disabled="busy || !dirty" class="primary">Salvar rascunho</button><span role="status" class="muted">{{ dirty ? 'Alterações ainda não salvas' : 'Rascunho salvo' }}</span></div>
            <button type="button" class="primary submit-message" :disabled="busy || (!draft.trim() && !workspace.attachments.length) || workspace.attachments.some(item => !item.available)" @click="run(submit)">Registrar mensagem local</button>
            <p class="muted">Ao trocar de campanha ou chat, o rascunho é salvo. Antes de fechar o aplicativo, use Salvar rascunho. Nada é enviado ao provedor.</p>
          </form>
        </template>
      </section>
      <aside v-if="workspace.configuration && !creating" class="workspace-panel configuration">
        <p class="eyebrow">CONFIGURAÇÃO DE ORIGEM</p><h2>{{ campaign?.name }}</h2>
        <dl><dt>Jogo</dt><dd>Victoria 3</dd><dt>Versão informada</dt><dd>{{ workspace.configuration.gameVersion }}</dd><dt>DLCs ativas confirmadas</dt><dd><ul v-if="workspace.configuration.activeDlcs.length"><li v-for="dlc in workspace.configuration.activeDlcs" :key="dlc">{{ dlc }}</li></ul><span v-else>Sem DLCs · confirmação explícita</span></dd><dt>Origem</dt><dd>Informada pelo jogador</dd></dl>
        <p class="muted">Configuração preservada com este chat. Não é uma observação automática do jogo nem garantia de suporte a essa versão. Alterações de versão e DLCs serão tratadas na subfase 2.4.</p>
      </aside>
    </div>
    <dialog ref="previewDialog" class="image-preview" aria-label="Prévia da captura" @click.self="closePreview" @close="preview = null">
      <button autofocus type="button" @click="closePreview">Fechar prévia</button>
      <img v-if="preview" :src="preview.previewDataUrl" :alt="preview.name" />
    </dialog>
  </section>
</template>

<style scoped>
.messages { list-style:none; padding:0; }
.messages > li { border-bottom:1px solid #46554b; padding:14px 0; }
.message-text { white-space:pre-wrap; }
.attachments { display:grid; grid-template-columns:repeat(2,minmax(0,1fr)); gap:12px; }
.attachments figure { min-width:0; margin:0; padding:8px; border:1px solid #46554b; border-radius:8px; }
.attachments figure > button:first-child { display:block; width:100%; padding:0; border:0; }
.attachments img { width:100%; height:120px; object-fit:contain; }
.attachments figcaption { font-size:11px; margin:8px 0; }
.submit-message { margin-top:16px; }
.image-preview[open] { position:fixed; margin:0; width:100vw; height:100vh; max-width:none; max-height:none; border:0; inset:0; z-index:20; background:rgba(0,0,0,.92); padding:24px; display:flex; flex-direction:column; align-items:center; gap:16px; }
.image-preview img { max-width:100%; min-height:0; max-height:calc(100vh - 100px); object-fit:contain; }
.workspace-grid { display:grid; grid-template-columns:210px minmax(0,1fr); gap:14px; align-items:start; }
.workspace-panel { min-width:0; padding:22px; border:1px solid #38443c; border-radius:15px; background:rgba(30,37,32,.88); overflow-wrap:anywhere; }
.conversation { background:rgba(24,30,27,.96); }
.configuration { grid-column:2; }
.sessions { display:flex; flex-direction:column; gap:12px; }
h2 { margin:0 0 18px; font-size:21px; font-weight:500; }
h3 { margin-top:24px; }
label { display:block; font-size:13px; margin-bottom:7px; }
input:not([type=checkbox]), select { width:100%; margin-bottom:16px; }
fieldset { border:0; padding:0; margin:0; min-width:0; }
.check { margin:16px 0; }
.check input, .dlc-list input { margin-right:7px; }
.muted, dt { color:#a8b3aa; font-size:12px; line-height:1.7; }
.chat-list, .dlc-list { list-style:none; margin:0; padding:0; }
.chat-list button { width:100%; text-align:left; margin-bottom:8px; }
.dlc-entry, .actions { display:flex; flex-wrap:wrap; gap:10px; align-items:center; }
.dlc-entry input { flex:1; min-width:100px; margin:0; }
.dlc-list li { display:flex; align-items:center; justify-content:space-between; gap:10px; margin:12px 0; }
.actions { margin-top:16px; }
.primary { background:#c3d8ae; color:#172314; }
.empty-chat { min-height:170px; display:flex; flex-direction:column; justify-content:center; color:#bdc7bf; font-size:14px; }
dd { margin:3px 0 18px; font-size:14px; } dd ul { padding-left:18px; }
@media(min-width:1200px) { .workspace-grid { grid-template-columns:210px minmax(0,1fr) 240px; } .configuration { grid-column:auto; } }
@media(max-width:700px) { .workspace-grid { grid-template-columns:1fr; } .configuration { grid-column:auto; } .workspace-panel { padding:18px; } }
</style>
