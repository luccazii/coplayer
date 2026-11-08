# Contratos da base local — 0.5

Consolidação inicial para macOS. O jogador adiou a 0.4/Legion; o núcleo não declara suporte Linux ou Gaming Mode. Os contratos de plataforma podem mudar após essa prova. Campanhas, chats e histórico de análises começam nas subfases 1.1–1.4.

## Responsabilidades

- `platform`: seleciona janela por ID/PID, consulta permissões e executa captura/foco pelo bridge macOS. Oculta e restaura as superfícies do Coplayer; fornece a imagem ao armazenamento. Não chama o provedor.
- `storage`: valida imagens, aplica limites, emite IDs, controla arquivos e catálogo SQLite. Não conhece janelas, autenticação ou análise.
- `provider`: gerencia app-server, autenticação e estados da análise. Resolve IDs de anexos importados no armazenamento; não aceita caminhos enviados pela interface. Uma análise ativa impede importação/remoção na sua coleção.
- `src/lib`: DTOs e chamadas Tauri. `Attachment` é compartilhado pelas duas provas; a interface apresenta o estado e não decide a validade do arquivo.

As coleções `capture` e `import` são independentes nesta etapa. A 1.2 fará a associação explícita entre anexos e mensagens de uma campanha/chat. Nenhuma captura do jogo é promovida automaticamente a entrada do modelo.

## IPC

`Attachment`: `id`, `name`, `mime`, `bytes`, `width`, `height`, `createdAtMs`, `available`, `previewDataUrl`. Sem caminhos de disco. Arquivo ausente ou com hash divergente mantém metadados visíveis, com `available=false` e preview vazio; o envio é rejeitado pelo núcleo.

`provider_images` recupera anexos importados; `platform_status` recupera capturas. `provider_import_images` abre o seletor nativo e adiciona um lote completo ou nenhum. Remoção requer ID e verifica a coleção. `provider_analyze` resolve IDs antes de iniciar a análise; duplicatas, IDs de outra coleção e arquivos adulterados são recusados. `platform-capture` só é emitido depois de persistir o anexo; a UI elimina duplicatas por ID.

`get_attachment_policy` fornece os limites vigentes para as duas telas. Comandos mutáveis retornam sucesso ou erro em português; as telas recarregam o estado autoritativo quando necessário. Os eventos do provedor continuam isolados por geração de processo, thread e turn, com estados terminais distintos. Reinício não reenvia análises nem inicia o runtime automaticamente.

Os próximos contratos de domínio deverão incluir `campaignId`, `chatId` e `requestId`, validados pelo Rust; IDs de contexto virão do núcleo. Esses campos não são adicionados ficticiamente às provas sem campanhas.

## Armazenamento e recuperação

Raiz: diretório local do aplicativo, subdiretório `data-v1/`:

- `coplayer.sqlite3`: catálogo, SHA-256 e estado de exclusão. Migração SQL transacional e versão em `PRAGMA user_version`; versão mais nova é recusada sem downgrade.
- `attachments/`: PNG/JPEG com nomes gerados internamente. Arquivos escritos e sincronizados antes do commit dos metadados. Falha no lote reverte o catálogo e remove arquivos criados pelo lote.
- `staging/`: saída temporária da captura nativa; removida após persistência/falha e reconciliada ao iniciar.

Diretórios privados `0700`, banco e anexos `0600` no macOS. SQLite embutido por `rusqlite`, sem instalação de serviço ou SQLite pelo jogador. Conexão com bloqueio exclusivo impede duas instâncias de reconciliar arquivos simultaneamente; tempo de espera de cinco segundos. A base não configura workspaces concorrentes nem sincronização.

Excluir primeiro grava um marcador no banco, depois remove o arquivo. Se houver interrupção, o anexo não reaparece; a próxima abertura tenta concluir a limpeza. Arquivos sem registro vivo são limpos somente quando têm nomes gerados por este catálogo dentro de suas pastas. Não há varredura ou limpeza das pastas antigas `proof-0.2/images` e `platform-proof/captures`.

O runtime continua em `proof-0.2/runtime-home` para preservar a autenticação isolada existente. Seus bancos/logs não pertencem ao catálogo Coplayer. Remover o arquivo gerenciado não comprova exclusão de cópias internas do runtime ou retenção remota; H08 completo permanece na 2.7.

## Limites locais

- Quatro anexos por coleção de prova.
- 8 MiB por imagem e 24 MiB por coleção.
- PNG/JPEG decodificáveis, no máximo 4096 pixels por lado; leitura e alocação limitadas.
- Captura nativa reduz a resolução proporcionalmente até 4096 pixels; importação acima do limite é rejeitada, sem alteração silenciosa do original.
- Pergunta: 16.000 bytes; resposta: 256 KiB, com falha explícita ao exceder.

São decisões locais para o protótipo; não representam limites publicados da assinatura. A associação futura de anexos a mensagens preservará a validação no núcleo.

## Empacotamento

`src-tauri/runtime-manifest.json` (macOS ARM64) e `src-tauri/runtime-manifest.linux-x64.json` (Linux x86_64) fixam versão, plataforma, artefato e hashes. O preparador seleciona o manifesto pelo ambiente de build e recusa plataformas não definidas; versão e hash esperado são compilados no adaptador Rust conforme o alvo. `npm run build:desktop` executa também `verify:bundle`, verificando identidade, mínimo macOS 14.0, executáveis e SHA-256 do runtime dentro do pacote. `npm run build:desktop:linux` extrai o AppImage para verificar executáveis ELF x86_64, hash do runtime e execução de `--version`. A prova Linux é inicial e não comprova assinatura, captura ou Gaming Mode.

Node é somente ferramenta de desenvolvimento/build. O aplicativo distribui Rust, WebView, SQLite e runtime fixado. Nenhuma prova desta etapa equivale à instalação limpa, assinatura/notarização Apple ou validação de Gaming Mode.

## Campanhas e chats — 1.1

Migração v2 adiciona `campaigns`, `campaign_configurations`, `chats` e `local_selection`, sem transformar anexos das provas em anexos de campanhas. A migração é transacional; falha preserva schema e dados anteriores, e schema futuro continua sendo recusado.

- `campaign_workspace` retorna campanhas, seleção, chats da campanha selecionada e configuração de origem do chat ativo. Nenhuma seleção é guardada no localStorage.
- `campaign_create` recebe nome, versão, nomes das DLCs ativas e as confirmações `dlcsConfirmed`/`noDlcsConfirmed`. O núcleo normaliza espaços, rejeita nomes vazios, duplicatas e limites excedidos. A lista vazia exige ambas as confirmações; uma lista preenchida rejeita confirmação contraditória de jogo sem DLCs. Nome (120 bytes), versão (80 bytes), até 64 DLCs (120 bytes cada).
- Criação de campanha, configuração inicial, primeira conversa e seleção acontece em uma transação. A configuração registra `source=player` e horário; é imutável por trigger. Não representa identificação visual nem suporte validado à versão informada.
- `campaign_select`, `chat_create` e `chat_select` retornam o estado autoritativo. IDs são gerados no Rust. Chaves estrangeiras compostas impedem associar chat/configuração/seleção a outra campanha.
- `chat_save_draft` exige campanha, chat e revisão esperada; atualizações antigas são rejeitadas. O rascunho admite até 16.000 bytes, preserva quebras de linha e não admite NUL. A interface salva explicitamente ou antes de trocar de chat/campanha; antes de encerrar, o jogador deve salvar alterações pendentes.
- `analysis_context` resolve e valida a configuração pelo par explícito campanha/chat, sem inferir contexto a partir da seleção global. É usado na leitura da sessão; o envio integrado deverá usar esse mesmo vínculo na 1.3.

Nesta etapa, chats preservam título, rascunho e configuração de origem; não há envio de mensagens nesses chats. A prova técnica 0.2 continua independente, efêmera e sem acessar contexto de campanhas. As subfases 1.2/1.3 integrarão anexos, pedidos e respostas persistentes; 2.4 adicionará mudanças de configuração e reavaliação. Não alterar registros históricos para implementar essas etapas.

## Composição e captura — 1.2

A migração v3 acrescenta `draft_generation` ao chat, `owner_chat_id`/`message_id` aos anexos e a tabela `messages`. Anexos anteriores continuam sem proprietário de chat e restritos às respectivas provas. Limites de quatro imagens, 8 MiB por imagem e 24 MiB totais passam a valer por rascunho, sem contar imagens de mensagens já registradas.

- `DraftTarget` contém `campaignId`, `chatId`, `generation`. `chat_capture` recebe esse destino; o atalho resolve o chat selecionado no núcleo antes da captura. A janela e o destino são mantidos durante a operação. A conclusão revalida a geração: se houve registro de mensagem durante a captura, a imagem atrasada é rejeitada e o staging removido. Se apenas mudou a seleção, a imagem permanece no chat original.
- O evento `chat-capture` identifica destino e anexo. A UI só adiciona a prévia ao rascunho correspondente; o evento não sobrescreve texto nem troca a seleção de campanha. O registro no catálogo é autoritativo, mesmo se a notificação não alcançar a janela.
- `chat_remove_attachment` valida campanha/chat/geração e só remove anexo do rascunho. Comandos de remoção/envio das provas não acessam imagens dos chats. Imagens históricas não são removíveis por esse comando; exclusão histórica permanece na 2.7.
- `chat_submit_message` recebe destino, revisão do texto, texto e IDs dos anexos revisados. Rejeita mensagem vazia, IDs duplicados/externos, lista desatualizada e arquivos ausentes/adulterados. Uma transação registra mensagem/configuração, associa anexos e limpa o rascunho, incrementando revisão e geração. Repetir a mesma geração/conteúdo retorna a mensagem existente; outro conteúdo é recusado.
- O estado persistido é `local_only`: mensagem registrada localmente, **ainda não enviada à IA**. Não é sucesso de análise, fila ou promessa de processamento posterior. Nenhum comando de captura/composição inicia runtime, provedor ou reenvio no reinício. A 1.3 deverá oferecer envio explícito e estados reais de análise, inclusive para qualquer mensagem local anterior que o jogador decidir enviar.
- Histórico retorna texto e contagem de anexos sem carregar todas as imagens em base64. `chat_message_attachments` carrega as prévias de uma mensagem explicitamente escolhida e verifica pertencimento, bytes e hash. Rascunhos preservam suas prévias limitadas e a UI permite ampliá-las.

Atalho ⌘⇧9 agora sempre captura para o chat selecionado, inclusive com outra aba aberta. Sem campanha/chat ele falha sem guardar uma imagem sem destino. A prova 0.3 continua capturando somente para sua coleção pelo botão próprio. Seleção da janela nativa continua efêmera por identidade ID/PID; após reiniciar é necessário selecionar a janela novamente.


O bundle macOS agora usa assinatura ad hoc explícita (`bundle.macOS.signingIdentity="-"`), conforme a [configuração oficial do Tauri](https://v2.tauri.app/distribute/sign/macos/#ad-hoc-signing). `verify:bundle` também verifica assinatura estrita, identificador `dev.coplayer.desktop`, vínculo do Info.plist e selo dos recursos, preservando o hash oficial do runtime. É um pacote local sem certificado Apple/notarização; autorizações do macOS podem precisar ser renovadas após rebuild. Uma assinatura válida não comprova autorização de captura.
