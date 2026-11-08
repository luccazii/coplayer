<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getAppInfo, isDesktop, type AppInfo } from "./lib/desktop";
import ProviderProof from "./components/ProviderProof.vue";
import PlatformProof from "./components/PlatformProof.vue";
import FloatingIcon from "./components/FloatingIcon.vue";

const iconSurface = new URLSearchParams(window.location.search).get("surface") === "icon";
const desktop = isDesktop();
const activeTab = ref<"provider" | "platform">("provider");
const info = ref<AppInfo | null>(null);
const error = ref("");
const unlisten: UnlistenFn[] = [];
let mounted = true;
onMounted(async () => {
  if (!desktop || iconSurface) return;
  try { info.value = await getAppInfo(); }
  catch (cause) { error.value = String(cause); }
  for (const name of ["platform-capture", "platform-error"]) {
    try {
      const stop = await listen(name, () => { activeTab.value = "platform"; });
      if (mounted) unlisten.push(stop);
      else stop();
    } catch (cause) { error.value = String(cause); }
  }
});
onUnmounted(() => { mounted = false; unlisten.forEach((stop) => stop()); });
</script>

<template>
  <FloatingIcon v-if="iconSurface" />
  <main v-else>
    <header>
      <span class="wordmark">coplayer<span aria-hidden="true">.</span></span>
      <span class="stage">Base local · 0.5</span>
    </header>
    <section class="intro">
      <p class="eyebrow">LABORATÓRIO LOCAL</p>
      <h1>Conectar, observar e verificar.</h1>
      <p class="description">Assinatura e anexos salvos neste dispositivo. A captura foi validada no macOS, em Victoria 3 no modo janela; campanhas e conversas serão adicionadas na próxima etapa.</p>
      <p v-if="info" class="native-status" role="status">Núcleo conectado · {{ info.platform }} / {{ info.architecture }}</p>
      <p v-else-if="error" role="alert">{{ error }}</p>
      <p v-if="info?.platform === 'linux'" class="notice" role="status">Prova inicial Linux · 0.4 em andamento. Captura isolada X11/XWayland, ícone e retorno de foco estão em avaliação. Gaming Mode, controle e teclado virtual ainda precisam de prova no aparelho.</p>
    </section>
    <p v-if="!desktop" class="notice">Prévia no navegador. Abra o aplicativo desktop para usar autenticação, captura e foco nativos.</p>
    <template v-else>
      <nav aria-label="Provas técnicas">
        <button type="button" :aria-pressed="activeTab === 'provider'" @click="activeTab = 'provider'">0.2 · Assinatura e imagens</button>
        <button type="button" :aria-pressed="activeTab === 'platform'" @click="activeTab = 'platform'">{{ info?.platform === 'linux' ? '0.4' : '0.3' }} · Captura e painel</button>
      </nav>
      <ProviderProof v-show="activeTab === 'provider'" />
      <PlatformProof v-show="activeTab === 'platform'" />
    </template>
    <footer>Provas experimentais. Campanhas, histórico persistente e fila pertencem às próximas subfases.</footer>
  </main>
</template>

<style>
:root { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: #e8e7e0; background: #161b19; font-synthesis: none; text-rendering: optimizeLegibility; -webkit-font-smoothing: antialiased; color-scheme: dark; }
* { box-sizing: border-box; }
body { margin: 0; }
main { max-width: 1120px; margin: 0 auto; padding: 28px 40px; }
header { display: flex; align-items: center; justify-content: space-between; gap: 16px; }
.wordmark { font-size: 26px; font-weight: 700; letter-spacing: -1px; }
.wordmark span { color: #bed594; }
.stage { color: #a8b3aa; font-size: 12px; text-align: right; }
.intro { margin: 30px 0 24px; }
.eyebrow { font-size: 11px; font-weight: 600; letter-spacing: 2px; color: #bed594; }
h1 { margin: 12px 0; font-size: clamp(26px, 4vw, 36px); font-weight: 500; line-height: 1.15; letter-spacing: -1px; }
.description { max-width: 740px; color: #abb6ae; font-size: 14px; line-height: 1.6; }
.native-status { font-size: 12px; color: #bed594; }
nav { display: flex; flex-wrap: wrap; gap: 10px; margin-bottom: 22px; }
h2 { font-size: 18px; font-weight: 600; }
h3 { font-size: 15px; }
p, label { line-height: 1.6; }
button, input, select, textarea { font: inherit; }
button { border: 1px solid #758765; border-radius: 6px; padding: 9px 13px; color: #e3edda; background: transparent; font-size: 13px; cursor: pointer; }
button:hover:not(:disabled), button[aria-pressed="true"] { background: #2d3a2a; }
button:focus-visible, input:focus-visible, select:focus-visible, textarea:focus-visible, a:focus-visible { outline: 2px solid #bed594; outline-offset: 3px; }
button:disabled { opacity: .5; cursor: not-allowed; }
input, select, textarea { color: #e8e7e0; background: #121814; border: 1px solid #4c5d50; border-radius: 5px; padding: 9px; max-width: 100%; }
textarea { width: 100%; resize: vertical; }
a { color: #bed594; overflow-wrap: anywhere; }
.notice { padding: 16px; border: 1px solid #627450; border-radius: 8px; }
footer { margin-top: 26px; color: #a8b3aa; font-size: 12px; line-height: 1.6; }
@media (max-width: 600px) { main { padding: 22px; } .stage { max-width: 140px; } }
</style>
