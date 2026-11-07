---
status: accepted
---

# Tauri com núcleo Rust e interface Vue

Em 2026-11-07, escolhemos Tauri 2, Rust, Vue 3 e TypeScript como base do Coplayer, com SQLite e capturas em arquivos locais. A composição preserva a facilidade de construir uma interface de chat e permite integrar recursos nativos sem distribuir Chromium; aceitamos o trabalho adicional de compatibilidade por plataforma em relação a Electron. A integração inicial será direta com Codex app-server empacotado, evitando acrescentar Python ao aplicativo.

A escolha é condicionada às provas de captura, foco, autenticação por assinatura e Gaming Mode descritas na [stack](../stack.md). Não representa suporte entregue nem vantagem de desempenho medida sobre Electron; se esses testes revelarem impedimentos relevantes, a decisão será revisitada.
