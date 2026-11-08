<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getAppInfo, isDesktop, type AppInfo } from "./lib/desktop";
import CampaignWorkspace from "./components/CampaignWorkspace.vue";
import ProviderProof from "./components/ProviderProof.vue";
import PlatformProof from "./components/PlatformProof.vue";
import FloatingIcon from "./components/FloatingIcon.vue";

const iconSurface = new URLSearchParams(window.location.search).get("surface") === "icon";
const desktop = isDesktop();
const activeTab = ref<"campaigns" | "provider" | "platform">("campaigns");
const info = ref<AppInfo | null>(null);
const error = ref("");
const unlisten: UnlistenFn[] = [];
let mounted = true;
onMounted(async () => {
  if (!desktop || iconSurface) return;
  try { info.value = await getAppInfo(); }
  catch (cause) { error.value = String(cause); }
  for (const name of ["platform-capture", "platform-error", "chat-capture", "chat-capture-error"]) {
    try {
      const stop = await getCurrentWebviewWindow().listen(name, () => { activeTab.value = name.startsWith("chat-") ? "campaigns" : "platform"; });
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
      <span class="stage">Capturas no chat · 1.2</span>
    </header>
    <section class="intro">
      <p class="eyebrow">LABORATÓRIO LOCAL</p>
      <h1>Cada partida, seu próprio contexto.</h1>
      <p class="description">Campanhas, configuração de origem e rascunhos salvos neste dispositivo. Capture para o chat e registre mensagens localmente; a análise pela assinatura vem na próxima etapa.</p>
      <p v-if="info" class="native-status" role="status">Núcleo conectado · {{ info.platform }} / {{ info.architecture }}</p>
      <p v-else-if="error" role="alert">{{ error }}</p>
      <p v-if="info?.platform === 'linux'" class="notice" role="status">Prova inicial Linux · 0.4 em andamento. Teste a janela, o teclado e a assinatura com imagens importadas. Captura, ícone flutuante e retorno de foco ao jogo ainda não estão implementados no Linux.</p>
    </section>
    <p v-if="!desktop" class="notice">Prévia no navegador. Abra o aplicativo desktop para usar autenticação, captura e foco nativos.</p>
    <template v-else>
      <nav aria-label="Áreas do aplicativo">
        <button type="button" :aria-pressed="activeTab === 'campaigns'" @click="activeTab = 'campaigns'">Campanhas e chats</button>
        <button type="button" :aria-pressed="activeTab === 'provider'" @click="activeTab = 'provider'">0.2 · Assinatura e imagens</button>
        <button type="button" :aria-pressed="activeTab === 'platform'" @click="activeTab = 'platform'">0.3 · Captura e painel</button>
      </nav>
      <CampaignWorkspace v-show="activeTab === 'campaigns'" />
      <ProviderProof v-show="activeTab === 'provider'" />
      <PlatformProof v-show="activeTab === 'platform'" />
    </template>
    <footer>Dados locais, sem sincronização. As provas técnicas ficam separadas das conversas. Envio integrado e histórico de análises pertencem à 1.3.</footer>
  </main>
</template>

<style>
:root { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color: #e8e7e0; background: #161b19; font-synthesis: none; text-rendering: optimizeLegibility; -webkit-font-smoothing: antialiased; color-scheme: dark; }
* { box-sizing: border-box; }
body { margin: 0; }
main { max-width: 1440px; margin: 0 auto; padding: 28px 40px; }
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
