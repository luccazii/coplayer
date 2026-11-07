# Stack do Coplayer

Consolidada em 2026-11-07. Base escolhida para o protótipo e MVP, condicionada às validações abaixo. A base Tauri/Vue/Rust 0.1 foi concluída com build macOS e IPC nativo verificados. Captura nativa e runtime de IA têm implementação experimental nas provas 0.2/0.3; aceitação pela assinatura e no jogo exige evidência separada. SQLite e medições completas permanecem posteriores. Evidências em [Plano por subfases](implementation-plan.md).

## Composição

| Camada | Escolha | Responsabilidade |
| --- | --- | --- |
| Aplicativo desktop | Tauri 2 | Janelas, ciclo de vida e integração desktop |
| Interface | Vue 3 + TypeScript | Chat, objetivos, configurações, resumo e apresentação de capturas |
| Núcleo local | Rust | Campanhas, contexto, fila global, persistência e coordenação dos provedores |
| Persistência | SQLite + arquivos locais | Dados estruturados no banco; imagens em arquivos referenciados |
| Integração inicial de IA | Codex app-server em processo auxiliar | Runtime empacotado por plataforma, controlado diretamente pelo núcleo Rust |
| Provedor inicial | OpenAI | Assinatura ChatGPT como pré-requisito; API Key como alternativa explícita |

O caminho inicial de integração direta com app-server substitui a preferência anterior por avaliar SDK Python. Não se adiciona um backend Python ou Node.js à distribuição inicial. Ferramentas de build da interface são distintas dos runtimes distribuídos ao jogador. Docker não é requisito de execução.

## Limites de responsabilidade

- A interface apresenta estado e envia comandos ao núcleo; regras de campanha, fila, contexto e retenção pertencem ao núcleo Rust.
- Captura, atalhos, foco e controle de janelas ficam em integrações por plataforma, separadas das regras de domínio.
- O adaptador OpenAI controla o processo Codex e traduz seus eventos. O domínio não depende diretamente dos tipos do protocolo, permitindo outros provedores posteriormente.
- Histórico, resumo e referências de imagens são propriedade do Coplayer. A persistência interna do Codex não substitui esse armazenamento nem comprova exclusão de imagens.
- O instalador deverá incluir o runtime necessário, sem exigir instalação manual do Codex. Versões, artefatos e fluxo de autenticação ainda precisam de validação.

## Motivação e alternativas

Tauri permite combinar núcleo Rust com uma interface web adequada a chat, Markdown, imagens e configurações, sem distribuir Chromium junto do aplicativo. Isso favorece uma distribuição menor, mas não comprova menor uso de RAM, CPU ou bateria.

Electron permanece alternativa se os testes mostrarem impedimentos relevantes. Slint, Iced e egui foram considerados para interface sem WebView; não foram escolhidos pelo trabalho adicional esperado para construir a experiência de chat. Qt também foi considerado, mas acrescentaria outra camada de integração ao núcleo Rust.

## Evidência e limites no Steam Deck

Pesquisa consultada em 2026-11-07; relatos não substituem testes do Coplayer:

- Um autor relata [cliente VPN Rust + Tauri funcionando em Desktop e Gaming Mode](https://www.reddit.com/r/SteamOS/comments/1vztdhp/i_ported_my_vpnclient_to_the_steamdeck_and_it_was/), após dificuldades de dependências e compilação. Isso não comprova overlay sobre outro aplicativo.
- O desenvolvedor de Perihelion relata [migração de Tauri para Electron](https://www.reddit.com/r/IndieDev/comments/1uztqcu/swapped_out_tauri_for_electron_to_get_decent/) para resolver problemas de desempenho e áudio no Linux em um jogo baseado em Canvas. A carga é diferente do chat, mas impede presumir que Tauri será sempre mais rápido.
- O projeto [tauri-steam-overlay-surface](https://github.com/PSG-Team/tauri-steam-overlay-surface) trata do overlay da Steam dentro de um aplicativo Tauri, com implementação descrita para Windows. Não comprova o requisito do Coplayer sobre Victoria 3 no Gaming Mode.
- A [arquitetura do Tauri](https://v2.tauri.app/concept/architecture/) e seu suporte a [executáveis auxiliares](https://v2.tauri.app/develop/sidecar/) sustentam a composição proposta, não sua compatibilidade final com o Deck.

## Validações antes de consolidar a implementação

1. Provar no Mac ARM64 captura sem o próprio overlay, ícone móvel, abertura por atalho e devolução do foco ao jogo.
2. Provar login, renovação, envio de múltiplas imagens, resposta e cancelamento pela assinatura usando uma versão publicada do runtime. API Key não substitui essa prova.
3. Avaliar cedo o Gaming Mode no Legion Go S com SteamOS disponível: sobreposição sobre o jogo, entrada, controle, teclado virtual, captura e foco. Validar Steam Deck separadamente; um aplicativo Linux que apenas abre não satisfaz esse requisito.
4. Medir o conjunto completo, incluindo Codex, com painel fechado, aberto e durante análise: RAM, CPU, impacto no jogo e consumo de energia quando possível.
5. Validar instalação em máquina sem ferramentas de desenvolvimento e definir distribuição por plataforma.

Uma VM Linux no Mac ajuda a testar dependências e funcionamento básico, mas não comprova desempenho ou compatibilidade gráfica do Deck. Um PC Linux com Gamescope fornece evidência intermediária. Sem acesso ao hardware, Gaming Mode permanece não verificado.

Versões da base serão registradas nos lockfiles e toolchain. Bibliotecas de captura e SQLite, migrações, autenticação concreta, transporte de análise, instaladores e atualização continuam em [Pendências técnicas](technical-readiness.md). RAG e demais provedores ficam para evolução posterior. A execução atual segue o plano aprovado, começando pela subfase 0.1.
