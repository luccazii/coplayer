# Prova de runtime e assinatura — 0.2

Subfase 0.2 concluída em 2026-11-07. No aplicativo nativo, a conta conectada concluiu uma análise com duas imagens sintéticas; outras execuções comprovaram cancelamento e falha por queda controlada do runtime, seguida de reconexão sem reenvio. As evidências e a inspeção de dados locais estão no [registro do plano](implementation-plan.md). Esta prova não implementa campanhas, fila nem histórico de análises; a base 0.5 adiciona catálogo SQLite de anexos e não valida qualidade sobre Victoria 3 ou instalação limpa.

## Executar

```sh
npm run prepare:runtime
npm run dev:desktop
```

Na aba da prova de assinatura:

1. Iniciar runtime. O aplicativo verifica o executável e inicia um processo exclusivo.
2. Conectar assinatura ChatGPT. O navegador abre o login gerenciado pelo app-server; o jogador conclui a autenticação. Não copiar credenciais do aplicativo Codex.
3. Selecionar um modelo visual e importar duas imagens sintéticas ou salvas. O botão de seleção usa o seletor nativo. Nenhuma imagem é enviada nessa etapa.
4. Escrever a pergunta e enviar explicitamente. A pergunta e as imagens selecionadas são compartilhadas com a OpenAI usando a assinatura conectada.
5. Verificar progresso, mensagem final e estado terminal. Fazer outra tentativa para testar cancelamento durante execução.
6. Encerrar o runtime. Reabrir e verificar autenticação persistida separadamente; nenhum pedido anterior é reenviado.

A prova aceita PNG/JPEG, até quatro imagens, 8 MiB por imagem e 24 MiB somados. São limites locais consolidados na 0.5, com máximo adicional de 4096 pixels por lado e decodificação PNG/JPEG; não são limites publicados do provedor. A pergunta tem limite de 16.000 bytes e a resposta, 256 KiB. Anexos são identificados por IDs emitidos pelo Rust; caminhos arbitrários não são aceitos no comando de envio.

## Runtime fixado

| Campo | Valor |
| --- | --- |
| Versão | Codex CLI 0.159.3 |
| Alvo desta prova | macOS ARM64 |
| Release publicada | [rust-v0.159.3](https://github.com/openai/codex/releases/tag/rust-v0.159.3), 2026-09-30 |
| Artefato | `codex-aarch64-apple-darwin.tar.gz` |
| SHA-256 do arquivo | `51de50a39ea592b5b0a64ae0474548c282181cb6b96d9bd08965a682ae12774c` |
| SHA-256 do executável | `4d210f7c5a18fd0386434df23b5bdbb8c0e7257d3e8a2b30b0769c8bbe99a878` |
| Destino de desenvolvimento | `src-tauri/runtime/codex`, ignorado pelo Git |
| Destino no bundle | recurso `runtime/codex` |

O script de preparação verifica os dois hashes. O aplicativo verifica o hash do executável antes de iniciar. Não procura `codex` no PATH nem depende do Codex instalado no computador do jogador. Node é usado somente na preparação/build; não integra o aplicativo distribuído. Outros alvos exigem registrar seus próprios artefatos e hashes antes de habilitar a prova.

Empacotar este runtime não conclui T04: instalação limpa, permissões, assinatura/notarização e regressão do pacote completo permanecem nas etapas previstas.

## Isolamento e configuração consultiva

O processo usa o diretório local do aplicativo, em `proof-0.2/runtime-home`, como `CODEX_HOME`. Em macOS, a raiz normalmente fica em `~/Library/Application Support/dev.coplayer.desktop/`. O diretório tem permissão `0700`. Configuração e credenciais usam esse diretório; o armazenamento de autenticação é forçado para arquivo e o método, para ChatGPT. O adaptador não lê/copía `~/.codex/auth.json`, não importa a configuração global e não recebe chaves de API do ambiente pai.

O app-server gerencia o login de navegador e a renovação de tokens; `account/read` com renovação é usado antes de cada análise. O adaptador exige tipo de conta `chatgpt` antes de iniciar um pedido. Não existe comando de API Key nesta prova.

A escolha é a autenticação gerenciada já exposta pelo app-server da versão fixada, para a prova pessoal local. Não implementa o fluxo independente de registro/compartilhamento de tokens de Sign in with ChatGPT e não comprova condições para distribuição pública.

A configuração é gerenciada pelo aplicativo: shell, execução unificada, code mode, apps, plugins, navegador, controle do computador, geração/leitura de imagens por ferramenta, múltiplos agentes, hooks e pesquisa web ficam desabilitados. Há sandbox de leitura e pedidos de aprovação/ferramentas vindos do processo são recusados; esse evento encerra a prova. As imagens chegam como entrada visual `localImage`, não por ferramenta. O diretório de trabalho é privado e vazio. Instruções de consulta substituem as instruções-base de programação para cada conversa.

Cada envio cria um thread efêmero independente. Identidades de campanha/chat do produto e contratos definitivos pertencem à 0.5 e às fases seguintes. `model/list` fornece um catálogo, não uma verificação de acesso da conta.

## Estados e falhas

- Somente `turn/completed` com estado `completed`, acompanhado de mensagem final completa e não vazia, produz sucesso.
- `interrupted` é cancelamento; `failed`, EOF ou quebra de transporte são falhas. Conteúdo parcial permanece distinguido de uma conclusão.
- Eventos de outro thread/turn ou de um processo anterior não alteram a prova atual.
- Cancelar durante a preparação impede o envio quando possível. Depois de `turn/start`, solicita `turn/interrupt` e aguarda o resultado terminal.
- RPCs têm timeout de 40 segundos. Uma falha ambígua ao iniciar a análise encerra o runtime, impedindo uma segunda execução enquanto a anterior possa estar ativa.
- Excesso do limite de resposta ou solicitação inesperada de ferramenta também encerra o runtime.
- Encerrar o aplicativo mata o processo auxiliar. Reiniciar nunca reenvia automaticamente o pedido.
- Limite de assinatura, perda de conexão e autenticação não provocam troca para API paga. A mensagem de falha orienta verificar conexão, autenticação e limites; distinção detalhada de limites/reconexão pertence à 2.7.

## Arquivos e evidências

A partir da 0.5, imagens importadas são copiadas para `data-v1/attachments` e catalogadas em SQLite, com IDs e previews recuperados após reinício. A UI não recebe caminhos. Arquivos ausentes ou com hash alterado não podem ser enviados. Remover desativa o registro e exclui o arquivo; isso não declara exclusão de cópias internas do runtime ou retenção remota. Cópias antigas em `proof-0.2/images` ficam preservadas e não são importadas automaticamente. Detalhes em [Contratos da base local](core-contracts.md); H08 completo pertence à 2.7. Nenhum dado local, credencial ou captura real deve entrar no Git.

Evidência inicial, anterior ao login: o artefato publicado foi baixado e os hashes conferiram; uma execução com diretório temporário isolado passou em `initialize`, retornou conta nula em `account/read` e listou oito modelos com entrada de imagem. Não houve mensagem em stderr. Ao terminar, o diretório isolado continha configuração e `installation_id`, sem credencial nem imagem. Na inspeção posterior à análise real, as cópias de entrada permaneceram em `proof-0.2/images`; bancos SQLite/WAL do runtime continham referências aos caminhos nos logs, mas não cópias binárias/base64 exatas das imagens testadas nem marcadores `data:image/`. Essa busca limitada não comprova retenção zero nem exclusão integral.

Testes automatizados do adaptador cobrem exclusão de eventos atrasados, estados terminais, rejeição de conta API Key, limite de saída, ausência de mensagem final, EOF e recusa de pedidos de ferramenta. Os subprocessos simulados desses testes exercitam apenas o transporte e nunca são oferecidos como provedor na interface.

Fontes: [protocolo oficial de app-server](https://developers.openai.com/codex/app-server), [configuração oficial](https://developers.openai.com/codex/config-reference) e schemas TypeScript gerados diretamente pelo executável 0.159.3. O código usa esse schema fixado, pois a documentação online pode evoluir.
