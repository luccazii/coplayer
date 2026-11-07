<script setup lang="ts">
import { ref } from "vue";
import { dragIcon, togglePanel } from "../lib/platform";

const error = ref("");
async function openPanel() {
  try { await togglePanel(); error.value = ""; }
  catch (cause) { error.value = String(cause); }
}
async function moveIcon(event: MouseEvent) {
  if (event.button !== 0) return;
  try { await dragIcon(); } catch (cause) { error.value = String(cause); }
}
</script>

<template>
  <div class="floating-icon" :title="error || 'Coplayer · abrir/fechar painel'">
    <div class="drag-handle" title="Arraste para reposicionar" @mousedown="moveIcon" aria-hidden="true">⠿</div>
    <button class="open-panel" type="button" aria-label="Abrir ou fechar painel do Coplayer" @click="openPanel">c<span>.</span></button>
    <span v-if="error" class="icon-error" role="alert">!</span>
  </div>
</template>

<style scoped>
.floating-icon { position: fixed; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; background: #1d2520; border: 1px solid #758765; border-radius: 12px; user-select: none; }
.drag-handle { cursor: move; color: #a8b3aa; height: 23px; width: 100%; text-align: center; font-size: 21px; line-height: 20px; }
.open-panel { padding: 0; margin: 0; width: 50px; height: 54px; border: 0; border-radius: 8px; font-size: 36px; line-height: 1; font-weight: 700; background: transparent; color: #e8e7e0; cursor: pointer; }
.open-panel span { color: #bed594; }
.open-panel:focus-visible { outline-offset: -4px; }
.icon-error { position: absolute; top: 1px; right: 5px; color: #ffb6a7; }
</style>
