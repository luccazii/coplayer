# Pendências e preparação técnica

Estado em 2026-11-07: produto e [stack](stack.md) consolidados; base 0.1 concluída, com build macOS e comunicação Vue–Rust verificados no aplicativo nativo. Prova 0.2 concluída com resposta real pela assinatura, cancelamento e falha/recuperação. Subfase 0.3 aceita pelo jogador no escopo de Victoria 3 em janela comum: captura limpa, painel, atalhos físicos, retorno de foco, remoção e reposicionamento do ícone verificados. Limitações não verificadas permanecem no plano. A prova no Legion (0.4) foi adiada por decisão explícita; consolidação macOS 0.5 concluída, com catálogo SQLite, anexos recuperados após reinício e regressão real pela assinatura. A próxima subfase é 1.1, campanhas e chats locais. Gaming Mode permanece não verificado. Evidências e pendências no [plano por subfases](implementation-plan.md).

## O que está decidido

- macOS primeiro, uso pessoal, janela sem bordas com alternativa em janela.
- Futuro suporte às quatro plataformas, incluindo Gaming Mode, com avaliação antecipada desse ambiente.
- Assinatura ChatGPT como pré-requisito; API Key opcional com escolha explícita.
- Tauri 2, Vue 3 e TypeScript na interface; núcleo local em Rust; SQLite e capturas em arquivos.
- Integração direta com Codex app-server empacotado, sem instalar Codex separadamente.
- Processos locais, sem backend Python ou Node.js distribuído e sem requisito de Docker.
- Assistência consultiva, captura manual, histórico local, uma análise global por vez.
- Repositório único; prova antecipada em Legion Go S com SteamOS. Evidência específica de Steam Deck continua separada.
- Fila global por envio; falha/cancelamento isolado por pedido, autenticação global pausa a fila e reinício não reenvia automaticamente. Preferências globais persistentes.

## O que ainda precisa ser definido ou validado

| Área | Decisão pendente | Evidência necessária |
| --- | --- | --- |
| Interface desktop | Validar Tauri 2 com Vue 3/TypeScript | Overlay, foco, ícone móvel, atalhos e captura limpa no macOS; perspectiva concreta para Gaming Mode |
| Núcleo local | Validar Rust controlando app-server diretamente | Custo de integração, autenticação, imagens, cancelamento e distribuição |
| Runtime | Versão e forma de distribuição | Artefatos publicados por plataforma e execução em máquina sem ambiente de desenvolvimento |
| Autenticação | Fluxo concreto e armazenamento de credenciais | Login da conta, renovação, limites, análise concluída pela assinatura e uso pessoal local sem homologação manual |
| Persistência | Catálogo de anexos com rusqlite/SQLite embutido, arquivos privados e migração versionada definidos na 0.5; esquema de campanhas na 1.1 | Histórico por campanha, correções, resumo e cópias internas do runtime permanecem posteriores |
| Captura | APIs e permissões por sistema | Excluir o próprio overlay e selecionar corretamente a janela do jogo |
| Transporte | Comunicação entre interface, serviço e runtime | Progresso, filas e interrupção sem confundir chats |
| Distribuição | Instalador, assinatura do aplicativo e atualização | Instalação autossuficiente e permissões necessárias no Mac alvo |
| Modelo | Modelo e parâmetros de análise | Disponibilidade real na conta, capacidade visual e qualidade nos casos do jogo |

A escolha de stack está registrada no [ADR 0001](adr/0001-desktop-stack.md). Essas pendências não reabrem automaticamente a escolha; impedimentos comprovados nas avaliações podem exigir revisão. Não há vantagem de desempenho medida sobre Electron.

## Conclusão das pesquisas disponíveis

Há caminhos documentados e projetos reais usando assinatura com frontend próprio e backend local. O SDK Python oferece runtime como dependência e login; TypeScript controla o CLI com imagens, mas exige atenção ao login e empacotamento. SDK Python usa app-server internamente. Esses fatos ajudam a selecionar candidatos, sem comprovar o produto instalado ou acesso da conta.

A preferência histórica por avaliar SDK foi sucedida pela escolha de integração direta com app-server pelo núcleo Rust. Sign in with ChatGPT permanece uma opção de autenticação, não uma decisão de linguagem nem uma alternativa necessariamente incompatível com app-server. Condições de futura comercialização não foram resolvidas e não devem ser extrapoladas do uso pessoal local.

Pesquisas datadas, sem execução:

- [Assinatura](chatgpt-subscription-feasibility.md)
- [SDK](codex-sdk-feasibility.md)
- [Integrações reais](subscription-integration-examples.md)

## Sequência aprovada de avaliação

0.1 prepara a base Tauri/Vue/Rust; 0.2 prova runtime e assinatura; 0.3 prova captura/foco no Mac; 0.4 avalia Gaming Mode no Legion; 0.5 consolida contratos com essas evidências. Um impedimento estrutural de plataforma exige revisão antes de ampliar o MVP. O jogador adiou explicitamente 0.4 para avançar no macOS; 0.5 consolida esse alvo, sem encerrar T05 nem presumir compatibilidade Linux. A implementação foi autorizada após o planejamento; publicação pública permanece fora da entrega.

Build e comunicação Vue–Rust não comprovam overlay, captura, assinatura ou instalação autossuficiente com runtime. Medições completas com o jogo e instalação limpa permanecem critérios posteriores do plano.

## Detalhes de produto ainda em aberto

Não exigem reiniciar a entrevista; resolver no desenho de cada fluxo, sem presumir escolhas do jogador:

- Recuperação detalhada dos pedidos interrompidos e apresentação da fila, respeitando a política já definida.
- Apresentação visual da recomendação fixada pendente de confirmação, sem tratá-la como aprovada nem manter a anterior como atual.
- Atalhos exatos; preferências de assistência já são globais e persistentes.
- Mac inicial ARM64 identificado; modelo, versões mínimas de sistema, múltiplos monitores e seleção da janela ainda precisam ser definidos.
- Conteúdo dos 16 casos iniciais, versões/DLCs cobertas e limiar de aprovação qualitativa, antes de 3.2.
- Limites de tamanho/quantidade de capturas e resposta a falha de envio.
- Sinalização de alerta baseado em captura antiga: frequência e forma de exibição sem novas observações.

Para etapas posteriores: algoritmo de relevância das imagens, chats antigos retomados na contagem de sessões, ponto exato de associação de capturas em ramificações, gesto de controle no Steam Deck, sincronização e distribuição pública.
