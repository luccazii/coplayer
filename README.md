# Coplayer

Copiloto consultivo para jogos de estratégia, inicialmente Victoria 3: observa capturas solicitadas pelo jogador, explica a situação e compara caminhos conforme suas prioridades.

## Estado do projeto

Subfase 0.1 concluída em 2026-11-07: base desktop Tauri 2, Rust, Vue 3 e TypeScript, com build macOS e comunicação Vue–Rust verificados no aplicativo nativo. Subfase 0.2 concluída: análise real pela assinatura, cancelamento e falha/recuperação verificados no aplicativo com runtime empacotado. Subfase 0.3 aceita pelo jogador no escopo de Victoria 3 em janela comum: captura limpa, painel, atalhos físicos, retorno de foco, remoção e reposicionamento do ícone verificados. Limitações não verificadas permanecem no plano. A prova no Legion (0.4) foi adiada por decisão explícita; consolidação macOS 0.5 concluída, com catálogo SQLite, anexos recuperados após reinício e regressão real pela assinatura. A próxima subfase é 1.1, campanhas e chats locais. O catálogo SQLite já preserva anexos; campanhas e Gaming Mode permanecem posteriores. Primeira entrega pessoal no macOS; avaliação antecipada em Legion Go S com SteamOS, sem presumir validação em Steam Deck.

## Desenvolvimento

A prova **0.4 foi retomada** para o Legion Go S com SteamOS. Preparação de pacote Linux via GitHub Actions e roteiro no [guia do Legion](docs/legion-proof.md). Captura, overlay e foco Linux ainda não estão implementados; a validação no aparelho permanece pendente.

```sh
nvm use
source "$HOME/.cargo/env"
npm ci
npm run prepare:runtime
npm run dev:desktop
```

Pré-requisitos, verificações e limites da base em [Desenvolvimento local](docs/development.md). `npm run dev` fornece apenas a prévia web; a conexão com Rust exige o aplicativo desktop.

## Documentação

| Documento | Finalidade |
| --- | --- |
| [Plano por subfases](docs/implementation-plan.md) | Entregas, dependências, critérios de conclusão e evidências de execução |
| [Desenvolvimento local](docs/development.md) | Ambiente, comandos e limites entre componentes |
| [Prova de assinatura](docs/runtime-proof.md) | Runtime fixado, autenticação isolada, imagens e estados terminais |
| [Contratos da base local](docs/core-contracts.md) | Anexos persistentes, IPC, limites, recuperação e manifesto de runtime |
| [Prova macOS](docs/platform-proof.md) | Captura, painel, foco, permissões e roteiro de validação |
| [Produto e MVP](docs/product-decisions.md) | Fonte canônica das decisões aprovadas e fluxos da primeira entrega |
| [Critérios de aceitação](docs/acceptance-criteria.md) | Cenários verificáveis para avaliar a primeira entrega |
| [Evolução posterior](docs/roadmap.md) | Regras aprovadas para depois do MVP e possibilidades ainda sem compromisso |
| [Stack](docs/stack.md) | Composição escolhida, responsabilidades, evidências e condições de validação |
| [Decisão de arquitetura](docs/adr/0001-desktop-stack.md) | Motivo da escolha de Tauri com Rust e Vue |
| [Pendências técnicas](docs/technical-readiness.md) | Avaliações e escolhas de implementação ainda abertas |
| [Glossário](CONTEXT.md) | Linguagem do domínio, sem especificação de implementação |

As pesquisas de [assinatura](docs/chatgpt-subscription-feasibility.md), [SDK](docs/codex-sdk-feasibility.md) e [integrações reais](docs/subscription-integration-examples.md) preservam evidências e recomendações datadas. Não representam decisões definitivas de arquitetura nem testes realizados. Em caso de divergência de escopo, prevalecem Produto e MVP e Evolução posterior; a stack vigente está em Stack e as escolhas restantes em Pendências técnicas.

A implementação foi iniciada após aprovação do plano, começando pela base 0.1. O jogador autorizou desenvolver 0.2 e 0.3 em paralelo, mantendo aceites separados. Publicação e distribuição pública não fazem parte desta entrega.
