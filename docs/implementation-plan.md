# Plano de implementação por subfases

Planejamento aprovado em 2026-11-07. Subfases **0.1–0.3 concluídas**; 0.3 aceita pelo jogador no escopo de janela comum, com limitações registradas. **0.5 concluída no macOS**; **0.4 retomada, em andamento**, para preparar e avaliar o pacote no Legion Go S com SteamOS. 1.1, campanhas e chats locais, continua pendente. Este documento organiza entregas; as regras de produto continuam em [Produto e MVP](product-decisions.md) e seus testes em [Aceitação](acceptance-criteria.md).

## Estratégia e decisões

- Entregas graduais: primeiro um fluxo estreito utilizável; depois completar o MVP macOS pessoal.
- Um repositório para todas as plataformas, com núcleo compartilhado e integrações nativas separadas. Tauri 2, Rust, Vue 3, TypeScript, SQLite e app-server empacotado permanecem a stack escolhida.
- Avaliar cedo Gaming Mode no **Legion Go S com SteamOS disponível ao jogador**. Evidência nesse aparelho não comprova Steam Deck. Impedimento estrutural exige revisão antes de ampliar o MVP; problemas pontuais ficam registrados para evolução.
- Uma subfase por chat de implementação. Execução sequencial por padrão; trabalho paralelo só após contratos estáveis e com componentes independentes.
- Não usar API Key como substituto da prova pela assinatura. Nenhuma resposta incompleta, cancelada ou falha conta como sucesso.

## Fase 0 — Caminhos críticos

| ID | Entrega / critério de conclusão | Dependências | Estado |
| --- | --- | --- | --- |
| 0.1 | Git, Tauri/Vue/TypeScript, núcleo Rust mínimo, comandos de desenvolvimento/verificação e janela nativa demonstrando IPC real. | Nenhuma | Concluída |
| 0.2 | Adaptador experimental Rust/app-server: autenticação isolada, múltiplas imagens, progresso, conclusão pela assinatura, cancelamento e falhas; versão publicada e artefatos registrados. | 0.1 | Concluída |
| 0.3 | Prova macOS: painel, retorno de foco, ícone reposicionável e captura limpa em Victoria 3 em janela comum; escopo aceito pelo jogador, demais cenários registrados para regressão. | 0.1 | Concluída |
| 0.4 | Prova no Legion Go S: overlay sobre jogo, captura, foco, controle e teclado virtual em Gaming Mode; evidência e impedimentos por capacidade. Pacote inicial Linux publicado; provas no aparelho pendentes. | 0.3 | Em andamento |
| 0.5 | Consolidar interfaces de plataforma/provedor, comunicação UI–Rust, persistência, arquivos, limites de anexos e empacotamento a partir das provas macOS; decisões Linux permanecem revisáveis até 0.4. | 0.2–0.3; 0.4 adiada | Concluída |

**Gate:** análise real bem-sucedida pela assinatura, captura/foco viáveis no Mac e direção arquitetural sustentada pela avaliação no Legion. Verificar configuração consultiva sem ferramentas desnecessárias, credenciais próprias e cópias de imagens no runtime. Registrar cada prova contra T01, T03 e T05 e cenários de O01–O03; os critérios completos só passam quando todos seus requisitos forem comprovados.

## Fase 1 — Primeiro protótipo utilizável

| ID | Entrega / critério de conclusão | Dependências | Estado |
| --- | --- | --- | --- |
| 1.1 | Criar/selecionar campanhas; exigir nome, versão e confirmação de DLCs antes da análise; persistir chats e configuração histórica. C01–C02. | 0.5 | Pendente |
| 1.2 | Capturar por botão/atalho, reunir/remover anexos e enviar explicitamente; captura nunca dispara análise. O01–O03. | 1.1 | Pendente |
| 1.3 | Pergunta e resposta real pela assinatura, progresso, cancelamento e estados terminais distintos. Uma execução por vez, ainda sem enfileiramento. T01/T03, C03/C05, A01/A06/A08/A09. | 1.2 | Pendente |
| 1.4 | Reabrir preservando campanhas, conversas e referências; pacote macOS com runtime incluído; fluxo completo durante uma partida. H07; primeira evidência de T04, ainda sem instalação limpa. | 1.3 | Pendente |

**Marco:** capturar, perguntar, receber orientação e retomar o histórico. Protótipo parcial, não MVP concluído. Desde 1.3, distinguir observações, inferências, declarações e lacunas, impedir mistura entre campanhas e pedir contexto para decisões importantes.

## Fase 2 — Comportamentos completos do MVP

| ID | Entrega / critério de conclusão | Dependências | Estado |
| --- | --- | --- | --- |
| 2.1 | Fila global, múltiplos chats, esclarecimentos vinculados e recuperação de interrupções. H01–H03. | 1.4 | Pendente |
| 2.2 | Objetivos, metas confirmadas, prioridades, conflitos e exceções confirmadas. A02/A04/A05. | 2.1 | Pendente |
| 2.3 | Contexto relevante para novos chats, origens/desatualização e resumo automático após sucesso. C04, A07, H04. | 2.2 | Pendente |
| 2.4 | Correções com vínculo histórico, mudança de versão/DLCs, reavaliação das orientações e nova campanha para save anterior. C06–C07, A07, H05. | 2.3 | Pendente |
| 2.5 | Fixar/desafixar e substituir orientações desatualizadas preservando histórico e confirmações necessárias. O04, H06. | 2.4 | Pendente |
| 2.6 | Perfis, controles independentes, interface/respostas PT-BR/EN, instruções textuais e alertas baseados nas capturas. O05, A02–A03, A08–A09. | 2.5 | Pendente |
| 2.7 | API Key explícita, limites/reconexão e exclusão manual de imagens, verificando cópias locais do runtime. T02–T03, H08. | 2.6 | Pendente |

**Marco:** todos os fluxos funcionais disponíveis para validação integrada.

## Fase 3 — Fechamento do MVP pessoal

| ID | Entrega / critério de conclusão | Dependências | Estado |
| --- | --- | --- | --- |
| 3.1 | Executar todos os critérios T/C/O/A/H, corrigir falhas e registrar evidências de isolamento, fila, cancelamento, correções e exclusão. | 2.7 | Pendente |
| 3.2 | Avaliação em economia, política interna, diplomacia e guerra, distinguindo erro visual, contexto ausente e desconhecimento das regras. | 3.1 | Pendente |
| 3.3 | Pacote sem Codex/ferramentas de desenvolvimento preinstalados; permissões, reinício, falha do runtime e medições com o jogo. T04 e regressão do fluxo instalado. | 3.2 | Pendente |

**Gate:** critérios aprovados com evidência, limitações registradas e nenhuma falha conhecida que misture campanhas, gere cobrança silenciosa ou declare análise interrompida como concluída. A instalação limpa de T04 só é concluída em 3.3; a matriz de 3.1 deve manter esse cenário pendente até então.

## Políticas definidas neste planejamento

- Fila global por ordem de envio. Falha/cancelamento afeta somente o pedido correspondente; falhas globais de autenticação pausam o processamento. Reinício não reenvia pedidos automaticamente.
- “Responder agora” suspende o avanço da fila e vincula a resposta à análise que pediu esclarecimento. “Continuar” libera a fila sem permitir orientação dependente da lacuna.
- Preferências de assistência globais e persistentes no MVP; idioma de resposta independente do idioma do jogo.
- Orientação fixada cuja substituição exige confirmação fica sinalizada como pendente, sem apresentar a nova ação como aprovada nem a anterior como atual.
- Avaliação inicial: 16 casos, cruzando quatro áreas do jogo, dois idiomas de tela e dois idiomas de resposta. Definir diagnóstico/lacunas esperados antes da execução, registrar erros e revalidar casos corrigidos. Versões, DLCs e limiar qualitativo final precisam ser fechados antes de executar 3.2.
- Medir latência, RAM e CPU com painel fechado, aberto e em análise; sem limite rígido antes das medições.

## Como executar cada subfase

Antes de editar, registrar objetivo, pré-requisitos, escopo, contratos afetados, critérios de aceite e comandos de validação. Ao concluir, registrar evidência, limitações e próximo passo. Estados permitidos: **Pendente**, **Em andamento**, **Bloqueada**, **Concluída**.

Os contratos definitivos de UI/núcleo, provedor, plataforma e persistência pertencem a 0.5. Cada pedido futuro deve carregar identidade de campanha/chat/pedido; o adaptador traduz eventos do runtime, e histórico/resumo pertencem ao Coplayer. Não consolidar esquema ou protocolo sem as provas das subfases anteriores.

Testes automatizados focam regras e contratos; simuladores servem a cenários determinísticos, nunca à prova de assinatura. Verificação nativa no jogo é obrigatória para captura/foco/atalhos. Builds e previews não comprovam esses comportamentos.

## Registro de execução

### 0.4 — Preparação do pacote Legion retomada em 2026-11-07

- **Autorização e alvo:** o jogador retomou a prova no Legion Go S com SteamOS e pediu preparar o pacote pelo Mac, possivelmente usando Docker. A 0.4 volta a **Em andamento**; 1.1 permanece pendente.
- **Objetivo desta etapa:** preparar build Linux x86_64 isolado via Docker, runtime da mesma versão publicada com hashes próprios e pacote AppImage para teste inicial no Desktop Mode. Não distribui Node/Python nem exige Docker no aparelho.
- **Escopo e contratos:** seleção explícita do runtime por plataforma, abertura do navegador Linux para autenticação isolada, verificação do pacote e roteiro de instalação/teste. Preservar o bundle macOS e os dados locais; não copiar autenticação ou imagens pessoais para o contexto Docker.
- **Aceite:** gates de frontend/Rust, build desktop e verificação de executáveis e recursos. A execução nativa no SteamOS, login, captura, foco, controle e teclado virtual requerem evidência no aparelho. O pacote inicial não implementa captura/overlay Linux e não conclui T05 nem a fase 0.4.
- **Pré-requisito observado:** Docker/OrbStack executa contêiner `linux/amd64` neste Mac ARM64. Apenas cerca de 2,5 GiB disponíveis no filesystem do Docker; compilação depende de espaço adicional. Nenhum cache, volume ou dado de outro projeto foi removido.
- **Direção atual:** o jogador preferiu GitHub Actions para compilar. Workflow `Legion Linux package` em Ubuntu 22.04 x86_64, gates e artefato AppImage com checksum e roteiro, sem release pública nem autenticação em CI. Docker permanece alternativa. O checkout ainda não tem commits nem remoto configurado; execução remota depende de definir/publicar o repositório.
- **Runtime Linux preliminar:** artefato oficial `codex-x86_64-unknown-linux-musl.tar.gz` da mesma versão 0.159.3. SHA-256 do arquivo conferido contra o digest da release; hash do executável fixado no manifesto Linux. Executado em contêiner x86_64 descartável: `codex-cli 0.159.3`. Não houve login nem inferência nesse contêiner.
- **Roteiro:** [Prova Legion](legion-proof.md) separa bootstrap, assinatura e capacidades ainda não implementadas de overlay/captura/foco no Linux. 0.4/T05 não concluídos.
- **Publicação autorizada:** o jogador escolheu repositório público e download de executável no Legion. Repositório `luccazii/coplayer` criado; pacote aprovado pela CI será disponibilizado como pré-release experimental. Isso excepciona a restrição anterior de publicação apenas para este repositório/pacote de prova, sem declarar distribuição estável.
- **Gates locais:** `npm run build`, `npm run check:rust`, `npm run test:rust` (19 testes) e `npm run build:desktop` passaram. Bundle macOS ARM64 e hash do runtime verificados. Aplicativo encerrado/reaberto: UI atualizada, `Núcleo conectado · macos / aarch64`, dois anexos sintéticos recuperados e runtime parado, sem reenvio. Scripts Node/shell e YAML verificados; seleção do manifesto rejeita arquitetura não definida. Build Linux e aceites do aparelho continuam pendentes até execução remota.
- **Primeira CI Linux:** frontend, Clippy e 19 testes passaram; executável e AppImage foram gerados, mas o verificador bloqueou publicação por hash do runtime alterado no empacotamento. Build passou a desativar o strip do linuxdeploy (`NO_STRIP=1`), preservando a verificação contra o manifesto oficial. O pacote com hash divergente não foi publicado. A CI macOS desse commit concluiu com sucesso.
- **Revisão solicitada antes da terceira execução:** a segunda CI também falhou no hash; `NO_STRIP` não resolveu. Revisados os fontes exatos do Tauri CLI 2.12.4 e linuxdeploy `07333c6`. O runtime musl é static PIE, com `PT_DYNAMIC` mas sem `DT_NEEDED`; o linuxdeploy o classifica como dinâmico e acrescenta `RUNPATH=$ORIGIN` ao varrer `usr/lib`.
- **Reprodução local isolada:** mesmo linuxdeploy publicado, Ubuntu 22.04 x86_64 em contêiner descartável e duas cópias do mesmo runtime. Mesmo com `NO_STRIP=1`, a cópia em `usr/lib` mudou de SHA-256 `8bf204b36a2f6dd0dab73aa2f639892e67ef9ac8befccb4a05b1496ebf25c479` para `38a8e1690ed0edb5199177b23f01554d16660d76f48568120bbdca363e5f54f3`; `readelf` confirmou o novo RUNPATH. A cópia em `usr/libexec` permaneceu byte a byte idêntica (`cmp`) e executou `codex-cli 0.159.3`. Sem autenticação e sem dados de campanha.
- **Correção revisada:** configuração específica Linux remove o recurso de `usr/lib` e o coloca em `usr/libexec/Coplayer/runtime/codex`; Rust resolve esse caminho, verificador rejeita cópia duplicada e conserva o hash oficial. `NO_STRIP` removido. Teste usa o parser real do Tauri para conferir merge Linux, esquema, caminho com espaços, rejeição de layouts inesperados e preservação dos recursos macOS. Frontend, Clippy, **20 testes** e bundle macOS passaram localmente. Nova CI somente após concluir a reprodução e revisão.
- **Prova de compactação antes de reenviar:** AppImage mínimo construído localmente pelo linuxdeploy exato, extraído e comparado ao runtime original com `cmp` e SHA-256. Runtime em `usr/libexec` preservou `8bf204b3…f25c479` e executou `codex-cli 0.159.3` após extração. Essa prova exercita empacotamento, não UI, assinatura ou Gaming Mode. Contêiner descartado; logs e arquivos de validação ficam apenas em `.local/validation/`, fora do Git. Revisão concluída antes da nova execução remota.
- **CI final e entrega em 2026-11-07 (America/Sao_Paulo):** commit `d70e72de02804126d3daed8c39b0e1d204e01ca4`, [Linux aprovado](https://github.com/luccazii/coplayer/actions/runs/42430241463) e [macOS aprovado](https://github.com/luccazii/coplayer/actions/runs/42430241465). Frontend, Clippy, **20 testes** e bundles passaram. AppImage de **171,17 MiB**, executáveis ELF x86_64, runtime extraído com hash oficial e `--version` aprovados; ausência de duplicata no caminho antigo verificada.
- **Pré-release publicada:** [v0.1.0-legion.1](https://github.com/luccazii/coplayer/releases/tag/v0.1.0-legion.1), apontando para o mesmo commit da CI. Download local conferido com `SHA256SUMS`; pacote `.tar.gz` inclui AppImage, launcher executável, roteiro e checksums. SHA-256 do AppImage: `df416961361e7513a8424d6effba73cad5f4858c61549049c6efc5d97e9cc2fb`; do arquivo `.tar.gz`: `ff50d81b1b1179ef713a629ba9976e84956f4bfa714c57ae6320b1f65bb231a4`. Digests dos assets publicados conferidos. Nenhuma credencial, imagem pessoal ou banco acompanha o pacote.
- **Próximo passo:** jogador executar o pacote no Desktop Mode do Legion e conferir janela/IPC, entrada, importação, reinício e assinatura; depois levantamento em Gaming Mode. Captura, overlay e foco Linux ainda não implementados; **0.4/T05 permanecem em andamento**, sem afirmar suporte a Steam Deck.

### Aceite 0.3 e avanço macOS — 2026-11-07

- O jogador confirmou que o funcionamento está satisfatório e autorizou avançar. A 0.3 foi aceita no escopo efetivamente demonstrado em janela comum. O teste sintético inconclusivo fica como regressão futura; janela sem bordas e comportamento com partida em execução continuam sem evidência, sem bloquear este avanço autorizado. Isso não declara O01–O03 completos.
- O jogador escolheu explicitamente adiar o Legion e avançar no protótipo macOS. A 0.4/T05 permanece pendente; a dependência da 0.5 foi excepcionada para este caminho. Contratos e empacotamento Linux deverão ser revistos após a prova real no aparelho.
- **Objetivo 0.5:** substituir armazenamento efêmero e limites divergentes por um catálogo SQLite de anexos e arquivos privados, utilizado pelas duas integrações reais, com contratos por ID e recuperação após reinício.
- **Escopo:** catálogo versionado, migração transacional, validação PNG/JPEG, limites centrais de quatro anexos por coleção de prova, 8 MiB por imagem, 24 MiB por coleção e 4096 pixels por lado; remoção recuperável pelo núcleo após interrupção e ausência de envio automático. Coleções de captura e importação permanecem separadas até a composição de mensagens na 1.2. Credenciais e arquivos antigos das provas ficam preservados, sem importação automática de campanhas reais.
- **Contratos:** UI recebe metadados/preview e usa IDs; caminhos ficam internos. Plataforma produz uma imagem, armazenamento valida/persiste, provedor resolve somente IDs importados. A futura 1.1 adiciona campanhas/chats ao banco via nova migração, sem criar entidades vazias nesta subfase.
- **Aceite e verificações:** reiniciar preserva anexos, IDs inválidos/arquivos ausentes ou adulterados não são enviados, lote inválido não deixa anexos parciais, exclusão não apaga arquivos externos; `npm run build`, `npm run check:rust`, testes de persistência e protocolo, `npm run build:desktop` e inspeção nativa. Runtime fixado e configuração consultiva permanecem vigentes.

### 0.5 — Consolidação local concluída em 2026-11-07

- **Implementação:** catálogo SQLite com migração transacional, bloqueio de instância, SHA-256, arquivos privados, validação real PNG/JPEG, limites compartilhados e recuperação de arquivos órfãos/exclusões interrompidas. Plataforma e provedor usam o mesmo `Attachment` por ID, em coleções separadas. Caminhos não são enviados à WebView. [Contratos e recuperação](core-contracts.md).
- **Empacotamento:** manifesto único para versão/hashes do runtime; `build:desktop` inclui `verify:bundle`. Pacote de **235,51 MiB**, executáveis ARM64, mínimo macOS 14.0 e hash do runtime verificados. Dependências resolvidas: `rusqlite 0.41.1` com SQLite embutido e `image 0.25.10` somente PNG/JPEG. Nenhum backend Node/Python distribuído.
- **Gates:** `npm run build`, `npm run check:rust`, `npm run test:rust` (**19 testes**), `npm run prepare:runtime` e `npm run build:desktop` passaram. Testes incluem reinício, transação interrompida, arquivos ausentes/adulterados, limite total/por arquivo/quantidade/dimensões, coleção incorreta, IDs duplicados, schema futuro e permissões Unix. Caches regeneráveis do projeto foram limpos entre os builds para respeitar o espaço disponível; dados de usuário e provas antigas preservados.
- **Importação nativa:** `fixture-a.png` e `fixture-b.png` importadas pelo seletor. Após encerrar e reabrir o pacote, ambas reapareceram com previews, sem iniciar runtime ou análise. Banco e três arquivos finais (duas importações e uma captura) observados com `0600`; pastas com `0700` e staging vazio.
- **Captura nativa:** janela do Preview exibindo a imagem sintética A capturada pelo botão, gerando anexo de **1800 × 1304**, com conteúdo esperado e sem painel Coplayer. Depois de novo encerramento/reabertura, a captura reapareceu; o alvo de captura permaneceu não selecionado, evitando reutilizar identidade de janela/PID entre processos. Nenhuma análise iniciou pela captura.
- **Regressão real do provedor:** `Iniciar runtime` recuperou a assinatura existente. Envio explícito dos dois anexos importados e recuperados chegou a **Concluída / Resposta concluída pela assinatura**, com `gpt-6.1-sol`: A = AZUL 42 / +100; B = VERDE 73 / +130. Nenhuma imagem da campanha real foi usada nessa regressão.
- **Permissão após rebuild:** inicialmente os Ajustes mostravam captura autorizada, mas o novo executável não reconhecia o acesso. Com autenticação Touch ID pelo jogador, o registro Coplayer foi recriado pela interface dos Ajustes para o pacote atual; após Encerrar e Reabrir, o app confirmou permissão e a captura passou. Não houve alteração programática de TCC. Essa limitação do pacote local fica documentada; assinatura/notarização e instalação limpa continuam posteriores.
- **Limites:** histórico de análises ainda é efêmero; catálogo de anexos não conclui H07/H08 de campanhas nem retenção remota. Diretórios antigos das provas não são importados/apagados automaticamente. Legion/T05, janela sem bordas e demais cenários dispensados não são declarados validados.
- **Conclusão:** escopo da 0.5 no macOS satisfeito. **Próximo passo: 1.1**, adicionar campanhas, configuração histórica de versão/DLCs e chats persistentes por nova migração, mantendo contexto validado pelo Rust.

### 0.2 e 0.3 — Desenvolvimento paralelo autorizado

- **Objetivo:** desenvolver a prova do provedor e a integração macOS sem exigir o jogo aberto durante o desenvolvimento.
- **Pré-requisito:** 0.1 concluída. Em 2026-11-07, o jogador solicitou trabalho paralelo nesta conversa, como exceção à sequência por chat. A aceitação de cada subfase permanece independente.
- **Divisão:** adaptador em `provider`, integração nativa em `platform`, componentes Vue separados e integração central dos comandos, recursos e janela. Nenhum componente depende diretamente da implementação do outro. Contratos experimentais, sem antecipar a consolidação de 0.5.
- **Escopo 0.2:** runtime publicado e fixado, credenciais próprias, imagens locais de teste, progresso, sucesso/falha/cancelamento e interface de prova. Sem API Key, campanhas, persistência de domínio ou fila.
- **Escopo 0.3:** janela alvo explícita, captura, painel, ícone móvel, atalhos e foco no macOS. Janela sintética serve apenas à validação técnica preliminar.
- **Aceite:** 0.2 exige inferência real concluída pela assinatura; simulador de protocolo comprova somente tratamento de eventos. 0.3 exige Victoria 3 em janela comum e sem bordas; teste sintético não conclui essa subfase.
- **Validação planejada:** `npm run build`, `npm run check:rust`, `npm run test:rust`, `npm run build:desktop`, inspeção do aplicativo nativo e registro separado das verificações que dependem de login, permissões ou jogo.
- **Implementação 0.2:** adaptador Rust de processo/JSON-RPC, autenticação gerenciada ChatGPT isolada, catálogo visual, importação nativa de até quatro imagens, progresso e estados terminais. Runtime publicado 0.159.3 para macOS ARM64 incluído como recurso e verificado por SHA-256. [Detalhes e hashes](runtime-proof.md).
- **Implementação 0.3:** bridge Objective-C ligado ao processo Coplayer, ScreenCaptureKit por identidade de janela/PID, painel com retorno de foco ao aplicativo selecionado, ícone móvel e atalhos. O pacote desta prova requer macOS 14+. [Detalhes e limites](platform-proof.md).
- **Verificações automatizadas em 2026-11-07:** `npm run build` passou; `npm run check:rust` passou; `npm run test:rust` passou com 11 testes. Incluem eventos atrasados, resposta sem mensagem final, EOF, recusa de ferramentas, estados terminais e exclusão mútua de captura/foco.
- **Ambiente de validação:** o disco ficou sem espaço durante as primeiras tentativas de teste/bundle. Apenas o cache `target/debug` deste projeto foi limpo; as verificações Rust foram refeitas com `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0`. A biblioteca desktop passou a gerar somente `rlib`, sem bibliotecas estáticas/dinâmicas de mobile. Não foram removidos dados pessoais nem caches de outros projetos.
- **Prova inicial do runtime:** executável publicado iniciou em diretório isolado, `initialize` passou, `account/read` retornou conta nula e `model/list` retornou oito modelos visuais. Isso não comprova assinatura. Duas imagens PNG sintéticas e uma janela Cocoa de teste foram preparadas em `.local/validation/`, ignorada pelo Git.
- **Bundle e inspeção nativa em 2026-11-07:** `npm run build:desktop` passou, gerando `Coplayer.app` de 233,53 MiB, com runtime incluído e mínimo macOS 14.0 confirmado no Info.plist. SHA-256 do executável dentro do bundle corresponde ao artefato fixado. A janela real em `tauri://localhost` mostrou `Núcleo conectado · macos / aarch64`; `Iniciar runtime` concluiu e habilitou conexão ChatGPT e seleção de modelo visual (`GPT-6.1-Sol`). Envio permanece indisponível sem conta autenticada.
- **Estado de autorização observado:** aba 0.3 mostrou `Permissão de Gravação da Tela necessária`, com captura e escolha de janela desabilitadas. Nenhuma captura foi realizada sem autorização. A janela sintética foi aberta, mas a validação de captura/foco aguarda a permissão do jogador.
- **Pendências naquele ponto, antes da participação do jogador:** login, análise real com múltiplas imagens, cancelamento conectado, inspeção de cópias do runtime e permissão de captura. O registro seguinte atualiza o resultado dessas verificações.

### 0.2/0.3 — Validação após autenticação pelo jogador

- **Autorização:** em 2026-11-07, o jogador concluiu o login e a permissão de Gravação da Tela. A inspeção nativa confirmou `ChatGPT conectado` e `Gravação da Tela permitida`.
- **T01 — prova real:** duas imagens sintéticas foram importadas pelo seletor nativo, sem iniciar análise. Após envio explícito, `gpt-6.1-sol` mostrou progresso e chegou ao estado **Concluída / Resposta concluída pela assinatura**. A resposta identificou tesouros 1200/1300, receitas 300/350, despesas 200/220, saldos +100/+130 e marcadores AZUL 42/VERDE 73. Calculou as diferenças corretamente e não presumiu sequência temporal ou uma partida real. Não foi usada API Key.
- **Cancelamento real:** uma segunda análise com as mesmas imagens iniciou geração; o botão de cancelamento produziu **Interrompida / Análise interrompida; conteúdo parcial não é uma conclusão** e liberou os controles. O texto parcial foi preservado sem ser apresentado como sucesso.
- **Falha e recuperação reais:** durante uma terceira geração, o processo auxiliar do Coplayer foi identificado por caminho exato e vínculo ao processo pai e encerrado de forma controlada. A interface apresentou **Falhou / Runtime desconectado; a análise não foi concluída**, preservando texto parcial. `Iniciar runtime` recuperou a assinatura salva e os controles; o resultado anterior permaneceu falho e não houve reenvio automático. Nenhum processo do aplicativo Codex foi encerrado.
- **Arquivos locais após inferência:** duas cópias PNG no diretório próprio `proof-0.2/images`; credencial própria `runtime-home/auth.json` presente com permissão `0600`, sem ler seu conteúdo. O runtime criou bancos auxiliares e assets de skills mesmo com ferramentas desabilitadas. Inspeção dos arquivos SQLite/WAL não encontrou cópias binárias/base64 exatas das duas imagens nem marcadores `data:image/`; encontrou referências aos caminhos locais em logs. Isso descreve esta execução efêmera, sem comprovar retenção zero, exclusão integral ou política remota.
- **Captura preliminar real:** seleção explícita de `Coplayer Test`, ícone ativado e captura pelo botão produziram anexo local de **1800 × 1344**, contendo apenas a janela sintética, sem painel ou ícone do Coplayer. O painel voltou e indicou que nenhuma análise foi iniciada pela captura. Esta evidência é preliminar e não conclui O01–O03 no jogo.
- **Atalho de captura físico:** após o teste pelo jogador, apareceu uma segunda captura às **18:26 (America/Sao_Paulo)** e o painel retornou à aba de captura. A imagem anterior era das 18:21, pelo botão. Isso confirma captura por teclado físico na janela sintética; as teclas sintetizadas pela automação não haviam acionado o atalho. A alternância por `⌘⇧Espaço` ainda precisa de confirmação específica; não é inferida da reabertura causada pela captura.
- **Conclusão 0.2:** aceite satisfeito por autenticação isolada, resposta terminal real com duas imagens, progresso, cancelamento e falha/recuperação, com runtime e artefatos fixados e dados locais inspecionados. **T01 verificado** neste ambiente; T03 completo (limites/perda de autenticação/alternativa explícita), instalação limpa e qualidade no jogo não são declarados concluídos.
- **Próximo passo 0.3:** completar a matriz preliminar de foco, reposicionamento, remoção de anexos, alvo fechado e atalho do painel; depois validar Victoria 3 em janela comum e sem bordas. A 0.3 permanece **Em andamento**.

### 0.3 — Validação com Victoria 3 em janela

- **Ambiente em 2026-11-07:** Victoria 3 macOS ARM64, versão Ivan-Chai 1.15.0, em modo `Janela`. Capturas de **1512 × 1038** pixels; o seletor apresenta dimensões lógicas de 756 × 519. A partida já estava pausada e permaneceu assim durante a inspeção.
- **Captura e painel:** duas capturas do jogo foram observadas na sessão, incluindo uma acionada pelo botão. As prévias contêm apenas a janela escolhida, sem painel ou ícone do Coplayer. O painel reabriu pelo ícone e apresentou `Captura adicionada localmente. Nenhuma análise foi iniciada.` Nenhuma imagem da campanha foi enviada ao provedor ou adicionada ao repositório.
- **Atalhos e foco no jogo:** o jogador executou fisicamente `⌘⇧Espaço` para fechar/reabrir o painel e `⌘⇧9` para capturar, confirmando que tudo funcionou, incluindo o retorno de foco. A inspeção posterior mostrou a terceira captura do jogo, também com 1512 × 1038 e sem superfícies do Coplayer. O resultado combina confirmação manual de foco com observação nativa do novo anexo; teclas sintetizadas pela automação não são evidência desses atalhos.
- **Remoção:** com autorização específica do jogador, a terceira captura do jogo foi removida pelo botão. A prévia desapareceu e a consulta ao diretório confirmou a ausência desse arquivo, preservando as duas capturas anteriores do jogo.
- **Modos de exibição:** o menu desta versão ofereceu somente `Janela` e `Tela Cheia`, sem opção explícita de janela sem bordas. O jogador pediu para pular o teste de tela cheia antes da alteração. O modo `Janela` foi preservado; tela cheia e janela sem bordas **não foram validadas**, sem tratar uma como equivalente à outra ou alterar o critério aprovado.
- **Alvo fechado — inconclusivo:** selecionada a janela sintética, a tentativa de fechamento foi seguida de timeout na inspeção dessa janela. O processo de teste permaneceu ativo e o sistema continuou listando a mesma janela após atualização. Uma captura subsequente gerou um anexo visualmente vazio de 1800 × 1344, em vez de demonstrar o erro esperado de alvo desaparecido. Investigar o ciclo de vida da janela de teste e a informação retornada pelo sistema antes de atribuir a causa ao Coplayer. A seleção foi restaurada para Victoria 3; o jogo não foi fechado.
- **Reposicionamento:** o gesto de arraste no puxador do ícone foi executado e o botão continuou abrindo o painel. O jogador confirmou visualmente que o ícone mudou de posição durante esse arraste. Reposicionamento verificado por execução do gesto e confirmação manual; a ferramenta fornece apenas imagem recortada da janela, sem posição na tela.
- **Ambiente:** a validação foi interrompida por falta de espaço no disco; após a limpeza realizada pelo jogador, a consulta mostrou 1,8 GiB livres e a inspeção nativa voltou a funcionar. Nenhuma alteração de código nesta rodada; os comandos de build e os 11 testes anteriormente aprovados não foram repetidos por se tratar de validação e documentação.
- **Estado:** **Em andamento**. Captura, alternância do painel, atalhos físicos, retorno de foco, remoção e reposicionamento do ícone têm evidência em janela comum. Restam a reprodução conclusiva de alvo desaparecido e a decisão futura sobre a cobertura de janela sem bordas, indisponível no menu observado. O comportamento ao abrir o painel com a partida em execução também não foi inferido deste teste com a partida pausada.

### 0.1 — Base de desenvolvimento

- **Objetivo:** aplicativo desktop mínimo executável com comunicação real entre Vue e Rust.
- **Escopo:** Git local, scaffold, toolchain, lockfiles, scripts, CI preparada e documentação. Sem banco, captura, login, IA ou publicação.
- **Contrato inicial:** comando `get_app_info` retorna `name`, `version`, `platform`, `architecture`; é diagnóstico local, não disponibilidade de provedor.
- **Aceite:** frontend tipado e compilado; Rust formatado e sem alertas Clippy; bundle macOS gerado; janela real mostra conexão e permite verificar novamente.
- **Validação:** `npm run build`, `npm run check:rust`, `npm run test:rust`, `npm run build:desktop` e inspeção nativa. Comandos detalhados em [Desenvolvimento](development.md).
- **Evidências em 2026-11-07:** `npm run build` passou; `npm run check:rust` passou sem avisos; `npm run test:rust` terminou com zero testes (nenhuma cobertura funcional alegada); `npm run build:desktop` gerou `Coplayer.app` (3,79 MiB nesta máquina, sem runtime de IA).
- **Inspeção nativa:** bundle aberto em macOS 26.1 ARM64, em `tauri://localhost`; estado observado: `Núcleo conectado · Coplayer 0.1.0 · macos / aarch64`. Clicar em `Verificar novamente` voltou ao estado conectado. Prévia web também inspecionada, identificando corretamente ausência do núcleo nativo e sem transbordamento horizontal.
- **Versões resolvidas:** Rust 1.99.1; Tauri Rust 2.12.5; CLI/API JS 2.12.4; Vue 3.5.45; TypeScript 6.0.3; Vite 8.3.6. Lockfiles preservam a resolução completa.
- **Limites:** CI apenas preparada, sem execução remota; nenhum critério de produto é declarado aprovado por este scaffold. Instalação limpa, assinatura/notarização Apple, IA, overlay/captura e Gaming Mode não foram validados.
- **Próximo passo:** 0.2, prova de runtime e assinatura; interação de login será feita pelo jogador quando necessária.

## Evolução após o MVP

1. Ramificações e retenção dinâmica conforme [roadmap](roadmap.md), fechando relevância e exclusão antes de implementar.
2. Status das recomendações confirmados pelo jogador.
3. Suporte utilizável no Legion, validação específica em Steam Deck e expansão Windows/Linux.
4. RAG e provedores adicionais.

Sincronização, voz, saves, distribuição pública e modelo comercial seguem sem compromisso de entrega. A ordenação acima é uma orientação de planejamento; o detalhamento só acontece após o MVP.
