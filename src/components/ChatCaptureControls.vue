<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listCaptureWindows, platformStatus, requestScreenPermission, selectCaptureWindow, setIconVisible, togglePanel, type PlatformStatus, type WindowTarget } from "../lib/platform";
const props = defineProps<{ disabled: boolean; atLimit: boolean }>();
const emit = defineEmits<{ capture: [] }>();
const status = ref<PlatformStatus | null>(null);
const windows = ref<WindowTarget[]>([]);
const selectedKey = ref("");
const busy = ref(false);
const error = ref("");
function key(window: WindowTarget) { return `${window.id}:${window.pid}`; }
async function refresh() {
  status.value = await platformStatus();
  selectedKey.value = status.value.selected ? key(status.value.selected) : "";
  windows.value = status.value.permissionGranted ? await listCaptureWindows() : [];
}
async function run(action: () => Promise<unknown>) {
  if (busy.value) return;
  busy.value = true; error.value = "";
  try { await action(); }
  catch (cause) { error.value = String(cause); }
  finally { busy.value = false; }
}
async function select() {
  const target = windows.value.find(window => key(window) === selectedKey.value);
  if (!target) return;
  try { await selectCaptureWindow(target); }
  finally { await refresh(); }
}
onMounted(() => run(refresh));
</script>
<template>
  <details class="capture-settings">
    <summary>Janela e controles de captura</summary>
    <p v-if="status && !status.supported">Captura disponível somente no macOS nesta etapa.</p>
    <template v-else>
      <p>{{ status?.permissionGranted ? 'Gravação da Tela permitida' : 'Permissão de Gravação da Tela necessária' }}</p>
      <div class="capture-actions">
        <button v-if="!status?.permissionGranted" type="button" :disabled="busy || props.disabled" @click="run(async () => { await requestScreenPermission(); await refresh(); })">Solicitar permissão de captura</button>
        <button type="button" :disabled="busy || props.disabled" @click="run(refresh)">Atualizar janelas de captura</button>
      </div>
      <label for="chat-capture-target">Janela do jogo</label>
      <select id="chat-capture-target" v-model="selectedKey" :disabled="busy || props.disabled || !windows.length" @change="run(select)">
        <option disabled value="">Escolha uma janela</option>
        <option v-for="window in windows" :key="key(window)" :value="key(window)">{{ window.appName }} · {{ window.title }}</option>
      </select>
      <p v-if="status?.selected">Selecionada: {{ status.selected.appName }} · {{ status.selected.title }}</p>
      <div class="capture-actions">
        <button type="button" :disabled="busy || props.disabled || props.atLimit || !status?.selected || !status.permissionGranted" @click="emit('capture')">Capturar para este chat</button>
        <button type="button" :disabled="busy || props.disabled || !status?.selected" @click="run(async () => { await setIconVisible(!status?.iconVisible); await refresh(); })">{{ status?.iconVisible ? 'Ocultar ícone flutuante' : 'Mostrar ícone flutuante' }}</button>
        <button type="button" :disabled="busy || props.disabled || !status?.selected" @click="run(togglePanel)">Voltar ao jogo</button>
      </div>
      <p>⌘⇧9 captura para o chat selecionado. ⌘⇧Espaço alterna o painel. Nenhum deles pausa o jogo ou envia uma análise.</p>
      <p v-if="status?.shortcutError" role="alert">{{ status.shortcutError }}</p>
    </template>
    <p v-if="error" role="alert">{{ error }}</p>
  </details>
</template>
<style scoped>
.capture-settings { border:1px solid #46554b; border-radius:8px; padding:12px; margin:16px 0; font-size:12px; }
summary { cursor:pointer; color:#c3d8ae; }
.capture-actions { display:flex; flex-wrap:wrap; gap:8px; }
label { display:block; margin:14px 0 6px; }
select { width:100%; }
</style>
