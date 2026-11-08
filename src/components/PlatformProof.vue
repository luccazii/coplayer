<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { attachmentPolicy, mebibytes, type AttachmentPolicy } from "../lib/attachments";
import { isDesktop } from "../lib/desktop";
import { captureWindow, listCaptureWindows, platformStatus, removeCapture, requestScreenPermission, selectCaptureWindow, setIconVisible, togglePanel, type CaptureArtifact, type PlatformStatus, type WindowTarget } from "../lib/platform";

const status = ref<PlatformStatus | null>(null);
const policy = ref<AttachmentPolicy | null>(null);
const windows = ref<WindowTarget[]>([]);
const selectedKey = ref("");
const captures = ref<CaptureArtifact[]>([]);
const busy = ref(false);
const error = ref("");
const message = ref("");
const subscriptions: UnlistenFn[] = [];
let disposed = false;

async function refreshStatus() {
  status.value = await platformStatus();
  captures.value = status.value.captures;
  selectedKey.value = status.value.selected ? key(status.value.selected) : "";
}
function key(window: WindowTarget) { return `${window.id}:${window.pid}`; }
async function perform(action: () => Promise<void>) {
  if (busy.value) return;
  busy.value = true;
  error.value = "";
  message.value = "";
  try { await action(); }
  catch (cause) { error.value = String(cause); }
  finally { busy.value = false; }
}
async function refreshWindows() {
  await refreshStatus();
  if (status.value?.permissionGranted) windows.value = await listCaptureWindows();
}
async function selectWindow() {
  const target = windows.value.find((window) => key(window) === selectedKey.value);
  if (!target) return;
  try {
    const selected = await selectCaptureWindow(target);
    if (status.value) status.value.selected = selected;
    selectedKey.value = key(selected);
    await refreshStatus();
  } catch (cause) {
    selectedKey.value = status.value?.selected ? key(status.value.selected) : "";
    throw cause;
  }
  message.value = `Janela selecionada: ${target.appName} · ${target.title}`;
}
function remember(capture: CaptureArtifact) {
  captures.value = [...captures.value.filter((item) => item.id !== capture.id), capture];
  message.value = "Captura adicionada localmente. Nenhuma análise foi iniciada.";
}
async function attach() { remember(await captureWindow()); }
async function remove(id: string) {
  try { await removeCapture(id); }
  finally { await refreshStatus(); }
  message.value = "Captura removida desta prova e seu arquivo local excluído.";
}
async function enableOverlay() {
  await setIconVisible(!status.value?.iconVisible);
  await refreshStatus();
}

onMounted(async () => {
  if (!isDesktop()) return;
  await perform(async () => {
    const captureListener = await listen<CaptureArtifact>("platform-capture", (event) => remember(event.payload));
    if (disposed) { captureListener(); return; }
    subscriptions.push(captureListener);
    const errorListener = await listen<string>("platform-error", (event) => { error.value = event.payload; });
    if (disposed) { errorListener(); return; }
    subscriptions.push(errorListener);
    policy.value = await attachmentPolicy();
    await refreshStatus();
  });
});
onBeforeUnmount(() => { disposed = true; subscriptions.forEach((unlisten) => unlisten()); });
</script>

<template>
  <section class="platform-proof" aria-labelledby="platform-title">
    <div class="proof-title"><h2 id="platform-title">Captura e painel no macOS</h2><span class="proof-label">Prova 0.3</span></div>
    <p class="muted">Selecione uma janela de teste ou do jogo. Capturas ficam neste dispositivo e só acontecem por comando seu.</p>
    <p v-if="!isDesktop()" role="status">Abra o aplicativo desktop para testar permissões, captura e foco. A prévia web não acessa essas funções.</p>
    <template v-else>
      <p v-if="status && !status.supported" role="status">Esta prova requer macOS 14 ou posterior.</p>
      <div class="permission-row">
        <span>{{ status?.permissionGranted ? 'Gravação da Tela permitida' : 'Permissão de Gravação da Tela necessária' }}</span>
        <button v-if="!status?.permissionGranted" :disabled="busy || !status?.supported" @click="perform(async () => { await requestScreenPermission(); await refreshWindows(); })">Solicitar permissão</button>
        <button :disabled="busy" @click="perform(refreshWindows)">Atualizar janelas</button>
      </div>
      <p v-if="status?.supported && !status.permissionGranted" class="muted">Após autorizar nos Ajustes do Sistema, atualize a lista. O macOS pode exigir encerrar e abrir o Coplayer novamente.</p>
      <label for="capture-target">Janela a capturar</label>
      <select id="capture-target" v-model="selectedKey" :disabled="busy || !windows.length" @change="perform(selectWindow)">
        <option value="" disabled>Atualize a lista e escolha uma janela</option>
        <option v-for="window in windows" :key="key(window)" :value="key(window)">{{ window.appName }} · {{ window.title }} ({{ Math.round(window.width) }} × {{ Math.round(window.height) }})</option>
      </select>
      <p v-if="status?.selected" class="selected-window">Selecionada: {{ status.selected.appName }} · {{ status.selected.title }}</p>
      <div class="actions">
        <button :disabled="busy || !status?.selected" @click="perform(attach)">Capturar janela</button>
        <button :disabled="busy || !status?.selected" @click="perform(enableOverlay)">{{ status?.iconVisible ? 'Desativar ícone flutuante' : 'Ativar ícone flutuante' }}</button>
        <button :disabled="busy || !status?.selected" @click="perform(async () => { await togglePanel(); })">Voltar ao aplicativo selecionado</button>
      </div>
      <p class="shortcuts">Painel: <kbd>⌘ ⇧ Espaço</kbd> · Captura para o chat selecionado: <kbd>⌘ ⇧ 9</kbd>. Use o botão acima para capturar nesta prova. Arraste o puxador do ícone para movê-lo.</p>
      <p v-if="status?.shortcutError" class="error" role="alert">Atalho indisponível: {{ status.shortcutError }}. Use os botões.</p>
      <p v-if="error" class="error" role="alert">{{ error }}</p>
      <p v-if="message" role="status">{{ message }}</p>
      <div v-if="captures.length" class="captures" aria-label="Capturas locais">
        <figure v-for="capture in captures" :key="capture.id">
          <img v-if="capture.available" :src="capture.previewDataUrl" :alt="`Captura de ${capture.name}`" />
          <p v-else role="alert">Arquivo ausente ou alterado.</p>
          <figcaption>{{ capture.name }} · {{ capture.width }} × {{ capture.height }}</figcaption>
          <button :disabled="busy" @click="perform(() => remove(capture.id))">Remover captura</button>
        </figure>
      </div>
      <p v-if="policy" class="muted">Até {{ policy.maxAttachments }} capturas, {{ mebibytes(policy.maxImageBytes) }} MiB por imagem, {{ mebibytes(policy.maxTotalBytes) }} MiB no total e {{ policy.maxDimension }} pixels por lado. Os anexos são recuperados ao reabrir o aplicativo e permanecem aqui até serem removidos.</p>
    </template>
    <p class="acceptance-note">Prova macOS aceita em Victoria 3 no modo janela. Tela cheia, janela sem bordas e Gaming Mode não foram validados. A devolução de foco ativa o aplicativo escolhido; múltiplas janelas do mesmo aplicativo exigem conferência manual.</p>
  </section>
</template>

<style scoped>
.platform-proof { border: 1px solid #38443c; background: #1d2520; padding: 24px; border-radius: 12px; margin-top: 20px; }
.proof-title, .permission-row, .actions { display: flex; gap: 12px; align-items: center; flex-wrap: wrap; }
.proof-title { justify-content: space-between; }
.proof-title h2 { font-size: 18px; }
.proof-label { color: #bed594; font-size: 12px; }
p { font-size: 13px; line-height: 1.6; overflow-wrap: anywhere; }
.muted, .shortcuts { color: #abb6ae; }
.permission-row { margin: 18px 0; font-size: 13px; }
label { display: block; font-size: 13px; margin: 18px 0 8px; }
select { width: 100%; min-height: 42px; padding: 8px; border: 1px solid #758765; border-radius: 6px; color: #e8e7e0; background: #161b19; font: inherit; font-size: 13px; }
.selected-window { color: #bed594; }
.error { color: #ffb6a7; }
kbd { padding: 3px 5px; border: 1px solid #586558; border-radius: 4px; white-space: nowrap; }
.captures { display: grid; grid-template-columns: repeat(auto-fit, minmax(210px, 1fr)); gap: 16px; margin: 20px 0; }
figure { min-width: 0; margin: 0; padding: 10px; background: #161b19; border-radius: 8px; }
img { width: 100%; max-height: 220px; object-fit: contain; }
figcaption { color: #abb6ae; font-size: 12px; margin: 8px 0; overflow-wrap: anywhere; }
.acceptance-note { padding-top: 16px; border-top: 1px solid #38443c; color: #abb6ae; margin-bottom: 0; }
</style>
