# Prova macOS — subfase 0.3

Implementação experimental de captura, painel e foco. O armazenamento e o DTO de anexo foram consolidados na [base local 0.5](core-contracts.md); a integração macOS permanece separada. Sem campanha, histórico de análises ou análise automática, recomendação fixada ou suporte declarado a Gaming Mode.

## Caminho implementado

- Vue chama comandos Rust; o núcleo coordena seleção, exclusão mútua de captura/foco, arquivos e eventos.
- O bridge Objective-C é compilado e ligado ao aplicativo. AppKit e ScreenCaptureKit executam dentro do processo Coplayer, sem helper externo nem runtime adicional.
- A prova requer macOS 14 ou posterior. O status distingue disponibilidade da API e autorização de Gravação da Tela.
- A seleção identifica uma janela por `windowID` e PID. A lista exclui janelas do Coplayer, áreas pequenas e camadas auxiliares. A captura consulta novamente o sistema e rejeita alvo desaparecido ou identidade divergente.
- `SCContentFilter(desktopIndependentWindow:)` captura apenas a janela escolhida. Antes de capturar, Rust oculta todas as superfícies Coplayer; após sucesso ou falha, restaura as superfícies e abre o painel. O cursor fica excluído.
- O ícone flutuante nasce oculto e é ativado explicitamente. O puxador superior permite reposicioná-lo. O painel fica acima das janelas comuns enquanto o ícone está ativo.
- `⌘⇧Espaço` abre/fecha o painel; `⌘⇧9` captura. Colisões de registro aparecem na interface, mantendo os botões disponíveis. A captura nunca chama o provedor.
- Fechar o painel com ícone ativo deve ocultá-lo e ativar o aplicativo selecionado. Sem ícone ativo, fechar a janela encerra o aplicativo. `⌘Q` continua sendo a saída completa.

## Limites e dados locais

A devolução de foco usa `NSRunningApplication.activate(from:options:)`. Isso ativa o aplicativo selecionado, mas não garante escolher uma janela secundária específica quando ele possui várias. O jogo deve ser conferido no teste real; esta prova não pede Acessibilidade nem injeta teclas ou cliques para levantar uma janela.

A base 0.5 limita a coleção a quatro capturas, 8 MiB por imagem, 24 MiB no total e 4096 pixels por lado. O núcleo fornece essa política à UI. Arquivos e metadados são persistidos em `data-v1/`, com diretórios `0700` e arquivos `0600`; a lista é recuperada após reinício. Arquivos ausentes/alterados são identificados, sem preview ou envio.

“Remover captura” desativa o registro e exclui seu arquivo; limpeza interrompida é retomada ao iniciar. O catálogo não importa nem remove automaticamente as capturas antigas em `platform-proof/captures`. Detalhes de recuperação e limites em [Contratos da base local](core-contracts.md). Isso não conclui H08, que inclui cópias do runtime de IA.

A permissão é solicitada apenas por ação explícita no botão correspondente. Se o macOS exigir reinício após autorizar, encerrar e reabrir o Coplayer e atualizar a lista. Nenhum teste usa alteração programática de TCC ou contorna a autorização do sistema.

## Validação

Comandos de integração obrigatórios: `npm run build`, `npm run check:rust`, `npm run test:rust` e `npm run build:desktop`. O teste unitário verifica exclusão mútua e liberação de uma operação interrompida; não comprova captura ou foco.

Verificação nativa preliminar pode usar uma janela sintética com marcadores conhecidos:

1. Abrir o bundle e conferir disponibilidade/estado de permissão, incluindo recusa.
2. Autorizar quando necessário, listar e selecionar a janela sintética.
3. Ativar ícone, movê-lo, abrir/fechar painel por clique e atalho e observar o foco.
4. Sobrepor o Coplayer à janela de teste; capturar pelo botão e pelo atalho. Conferir somente o conteúdo esperado, sem painel ou ícone.
5. Reunir duas capturas e remover uma. Conferir que nenhum pedido de IA é disparado.
6. Fechar a janela selecionada e tentar capturar: erro claro, painel restaurado e nenhuma imagem de outra janela.
7. Repetir as operações com Victoria 3 em janela comum e sem bordas antes de concluir 0.3. Registrar eventuais diferenças de tela/monitores e permissões.

Build, preview web e janela sintética não concluem O01–O03 nem a subfase. As evidências efetivamente executadas ficam no [registro do plano](implementation-plan.md).

## Fontes consultadas

- [Apple: ScreenCaptureKit](https://developer.apple.com/documentation/screencapturekit) e headers do SDK local: seleção de janela, autorização e isolamento da captura.
- [Apple: SCScreenshotManager](https://developer.apple.com/documentation/screencapturekit/scscreenshotmanager): captura de um frame.
- [Apple: NSRunningApplication](https://developer.apple.com/documentation/appkit/nsrunningapplication): ativação do aplicativo escolhido.
- [Tauri 2: Global Shortcut](https://v2.tauri.app/plugin/global-shortcut/): registro e eventos nativos de atalhos.

A prova 0.3 foi aceita pelo jogador em janela comum. Janela sem bordas não apareceu no menu do Victoria 3 observado; tela cheia foi dispensada. O roteiro acima preserva os cenários ainda não demonstrados para regressão futura.
