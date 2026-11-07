<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { attachmentPolicy, mebibytes, type AttachmentPolicy } from "../lib/attachments";
import { isDesktop } from "../lib/desktop";
import {
  providerAnalyze, providerCancel, providerCancelLogin, providerImportImages,
  providerLogin, providerImages, providerRemoveImage, providerStart, providerStatus, providerStop,
  type ProofImage, type ProviderStatus,
} from "../lib/provider";

const desktop = isDesktop();
const status = ref<ProviderStatus | null>(null);
const images = ref<ProofImage[]>([]);
const policy = ref<AttachmentPolicy | null>(null);
const prompt = ref("Compare as imagens anexadas. Liste o que você observa em cada uma, as diferenças e o que não é possível concluir apenas com elas.");
const model = ref("");
const busy = ref(false);
const error = ref("");
let pollTimer: ReturnType<typeof setTimeout> | undefined;
let disposed = false;
let polling = false;
const active = computed(() => ["starting", "running", "cancelling"].includes(status.value?.analysis?.status ?? ""));
const ready = computed(() => status.value?.runtime === "ready");
const resultLabel = computed(() => {
  const labels = { starting: "Preparando", running: "Em andamento", cancelling: "Cancelamento solicitado", completed: "Concluída", cancelled: "Interrompida", failed: "Falhou" };
  return status.value?.analysis ? labels[status.value.analysis.status] : "";
});

async function refresh() {
  if (!desktop || polling || disposed) return;
  polling = true;
  try {
    status.value = await providerStatus();
    if (!status.value.models.some((option) => option.id === model.value)) {
      model.value = status.value.models.find((option) => option.isDefault)?.id ?? status.value.models[0]?.id ?? "";
    }
  } catch (cause) { error.value = String(cause); }
  finally { polling = false; }
}
async function poll() {
  await refresh();
  if (!disposed) pollTimer = setTimeout(poll, 650);
}
async function run(action: () => Promise<unknown>) {
  if (busy.value) return;
  busy.value = true;
  error.value = "";
  try { await action(); }
  catch (cause) { error.value = String(cause); }
  finally { busy.value = false; await refresh(); }
}
async function interrupt() {
  // Cancellation and stop remain usable while turn/start is waiting for RPC.
  error.value = "";
  try { await providerCancel(); }
  catch (cause) { error.value = String(cause); }
  await refresh();
}
async function stop() {
  error.value = "";
  try { await providerStop(); }
  catch (cause) { error.value = String(cause); }
  await refresh();
}
async function importImages() { await providerImportImages(); images.value = await providerImages(); }
async function removeImage(id: string) {
  try { await providerRemoveImage(id); }
  finally { images.value = await providerImages(); }
}
onMounted(async () => {
  if (!desktop) return;
  try { [images.value, policy.value] = await Promise.all([providerImages(), attachmentPolicy()]); }
  catch (cause) { error.value = String(cause); }
  await poll();
});
onUnmounted(() => { disposed = true; if (pollTimer) clearTimeout(pollTimer); });
</script>

<template>
  <section class="provider-proof" aria-labelledby="provider-title">
    <div class="proof-heading">
      <div><p class="proof-kicker">PROVA 0.2</p><h2 id="provider-title">Análise pela sua assinatura</h2></div>
      <span class="proof-badge">{{ status?.authenticated ? "ChatGPT conectado" : "Não conectado" }}</span>
    </div>
    <p class="proof-copy">Teste texto e imagens com o provedor real, sem precisar abrir o jogo. Esta prova não cria campanhas nem valida a qualidade de orientação sobre Victoria 3.</p>
    <p v-if="!desktop" class="proof-notice">Abra o aplicativo desktop para iniciar o runtime e conectar sua assinatura.</p>
    <template v-else>
      <div class="proof-actions">
        <button type="button" :disabled="busy || ready || status?.runtime === 'starting'" @click="run(providerStart)">Iniciar runtime</button>
        <button type="button" :disabled="busy || !ready || active || status?.loginPending" @click="run(providerLogin)">{{ status?.authenticated ? "Reconectar assinatura" : "Conectar assinatura ChatGPT" }}</button>
        <button v-if="status?.loginPending" type="button" :disabled="busy" @click="run(providerCancelLogin)">Cancelar login</button>
        <button v-if="ready || status?.runtime === 'starting'" type="button" @click="stop">Encerrar runtime</button>
      </div>
      <p class="proof-status" role="status" aria-live="polite">{{ status?.message ?? "Consultando o núcleo…" }}</p>
      <p class="proof-meta">Runtime {{ status?.runtimeVersion ?? "0.159.3" }} · credenciais isoladas · sem troca automática para API paga</p>
      <form @submit.prevent="run(() => providerAnalyze(prompt, model, images.map((image) => image.id)))">
        <label for="proof-model">Modelo visual</label>
        <select id="proof-model" v-model="model" :disabled="!ready || busy || active">
          <option value="" disabled>{{ ready ? "Nenhum modelo visual disponível" : "Inicie o runtime para consultar" }}</option>
          <option v-for="option in status?.models ?? []" :key="option.id" :value="option.id">{{ option.name }}</option>
        </select>
        <p class="proof-meta">A lista de modelos não comprova acesso; a resposta concluída confirma esta tentativa.</p>
        <div class="proof-image-heading">
          <label>Imagens de teste · {{ images.length }}/{{ policy?.maxAttachments ?? "…" }}</label>
          <button type="button" :disabled="busy || active || !policy || images.length >= policy.maxAttachments" @click="run(importImages)">Selecionar imagens</button>
        </div>
        <p v-if="policy" class="proof-meta">PNG ou JPEG · até {{ mebibytes(policy.maxImageBytes) }} MiB por imagem, {{ mebibytes(policy.maxTotalBytes) }} MiB no total e {{ policy.maxDimension }} pixels por lado. Anexos ficam salvos neste dispositivo; selecionar não envia ao provedor.</p>
        <ul v-if="images.length" class="proof-images">
          <li v-for="image in images" :key="image.id">
            <img v-if="image.available" :src="image.previewDataUrl" :alt="image.name" />
            <p v-else role="alert">Arquivo ausente ou alterado. Remova e importe novamente.</p>
            <span>{{ image.name }}</span>
            <button type="button" :disabled="busy || active" :aria-label="`Remover ${image.name}`" @click="run(() => removeImage(image.id))">Remover</button>
          </li>
        </ul>
        <label for="proof-prompt">Pergunta</label>
        <textarea id="proof-prompt" v-model="prompt" rows="4" maxlength="16000" :disabled="busy || active"></textarea>
        <p class="proof-copy">Ao enviar, sua pergunta e as imagens selecionadas serão compartilhadas com a OpenAI usando a assinatura conectada. Cópias locais de teste e dados internos do runtime podem permanecer no dispositivo.</p>
        <div class="proof-actions">
          <button class="proof-primary" type="submit" :disabled="busy || active || !ready || !status?.authenticated || !model || !prompt.trim() || images.some((image) => !image.available)">Enviar prova pela assinatura</button>
          <button v-if="active" type="button" :disabled="status?.analysis?.status === 'cancelling'" @click="interrupt">Cancelar análise</button>
        </div>
      </form>
      <p v-if="error" class="proof-error" role="alert">{{ error }}</p>
      <section v-if="status?.analysis" class="proof-result" aria-labelledby="proof-result-title">
        <h3 id="proof-result-title">{{ resultLabel }}</h3>
        <p class="proof-status" role="status">{{ status.analysis.message }}</p>
        <p class="proof-meta">{{ status.analysis.model }} · {{ status.analysis.imageCount }} imagem(ns)</p>
        <pre v-if="status.analysis.text">{{ status.analysis.text }}</pre>
        <p v-else class="proof-meta">Nenhuma resposta recebida ainda.</p>
      </section>
    </template>
  </section>
</template>

<style scoped>
.provider-proof { padding: 26px; border: 1px solid #38443c; border-radius: 12px; background: #1d2520; }
.proof-heading { display: flex; justify-content: space-between; align-items: start; gap: 20px; }
.proof-kicker { margin: 0 0 8px; color: #bed594; font-size: 10px; letter-spacing: 1.8px; }
h2 { margin: 0; font-size: 21px; font-weight: 500; }
h3 { font-size: 15px; margin: 0 0 12px; }
.proof-badge { font-size: 11px; color: #c6d7ba; white-space: nowrap; border: 1px solid #516149; border-radius: 20px; padding: 6px 10px; }
.proof-copy, .proof-status, .proof-notice { color: #bdc7bf; font-size: 13px; line-height: 1.7; }
.proof-meta { font-size: 11px; color: #9cae9f; line-height: 1.6; }
.proof-actions { display: flex; flex-wrap: wrap; gap: 8px; }
form { margin-top: 26px; padding-top: 22px; border-top: 1px solid #38443c; }
label { display: block; font-size: 12px; margin: 0 0 8px; color: #d5dfd7; }
select, textarea { width: 100%; font: inherit; font-size: 13px; background: #141b17; color: #e8e7e0; border: 1px solid #46554b; border-radius: 6px; padding: 11px; }
textarea { line-height: 1.6; resize: vertical; min-height: 100px; }
select:focus-visible, textarea:focus-visible { outline: 2px solid #bed594; outline-offset: 2px; }
.proof-image-heading { display: flex; align-items: center; justify-content: space-between; gap: 16px; margin-top: 22px; }
.proof-image-heading label { margin: 0; }
.proof-images { list-style: none; padding: 0; display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 12px; }
.proof-images li { min-width: 0; border: 1px solid #38443c; padding: 10px; border-radius: 6px; }
.proof-images img { width: 100%; height: 100px; object-fit: contain; background: #111714; }
.proof-images span { display: block; overflow-wrap: anywhere; font-size: 11px; margin: 8px 0; }
.proof-primary { background: #b8ce93; color: #172314; border-color: #b8ce93; }
.proof-primary:hover:not(:disabled) { background: #ccdda9; }
.proof-error { background: #382823; padding: 12px; border-radius: 6px; color: #f4c6b8; font-size: 13px; line-height: 1.5; }
.proof-result { margin-top: 24px; border-top: 1px solid #46554b; padding-top: 22px; }
pre { white-space: pre-wrap; overflow-wrap: anywhere; font: inherit; font-size: 14px; line-height: 1.8; }
@media (max-width: 600px) { .provider-proof { padding: 18px; } .proof-heading { flex-direction: column; gap: 12px; } }
</style>
