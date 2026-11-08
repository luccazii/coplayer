# Desenvolvimento local

## Base atual

Base e provas 0.1–0.3 aceitas; consolidação 0.5 concluída no macOS. Legion/0.4 retomado pelo jogador, com preparação de pacote Linux via GitHub Actions; veja o [roteiro do Legion](legion-proof.md). O aplicativo desktop Tauri 2 apresenta duas provas independentes: assinatura com imagens importadas e captura/painel macOS. A janela consulta `get_app_info` para verificar o núcleo. No navegador, a tela informa que é somente uma prévia e não apresenta controles nativos operáveis.

Há implementação experimental de provedor e plataforma, com catálogo SQLite de anexos e sem campanhas. Os ícones são provisórios, provenientes do template oficial. Um build não comprova acesso ao modelo, captura, foco ou sobreposição no jogo. Veja os registros de execução no plano para distinguir código de evidência.

## Ambiente e execução

Primeiro alvo: macOS ARM64. Node está fixado em `.nvmrc`, Rust em `rust-toolchain.toml`, e as dependências resolvidas em `package-lock.json` e `src-tauri/Cargo.lock`. O identificador local provisório é `dev.coplayer.desktop`; não implica domínio registrado ou distribuição pública.

Pré-requisitos: Node/npm, Rust via rustup e ferramentas de desenvolvimento Apple. Consulte os [pré-requisitos oficiais do Tauri](https://v2.tauri.app/start/prerequisites/).

```sh
nvm use
source "$HOME/.cargo/env"
rustup show
npm ci
npm run prepare:runtime
npm run dev:desktop
```

O bootstrap inicial instalou Rust sem modificar o perfil do shell. Carregar `~/.cargo/env` disponibiliza Cargo nessa sessão. Em ambientes onde Cargo já está no PATH, esse passo é dispensável.

`npm run dev` abre somente o servidor de interface em `127.0.0.1:1420`; a conexão com Rust requer `npm run dev:desktop`. A porta é fixa e o processo falha se estiver ocupada, evitando conectar silenciosamente à instância errada. Encerrar com Ctrl+C. O desenvolvimento desta base ainda não configura portas independentes por worktree.

`prepare:runtime` obtém o artefato publicado e fixado do Codex para macOS ARM64, verifica seu hash e salva o executável em `src-tauri/runtime/` (ignorado pelo Git). Ele precisa existir antes das verificações Rust e do empacotamento. O runtime distribuído não depende do Codex instalado no computador. As provas nativas de captura exigem macOS 14 ou posterior e permissão de Gravação da Tela para Coplayer. Nenhum fluxo deve reutilizar o diretório de autenticação de outro aplicativo.

O bundle experimental tem mínimo macOS 14.0. A biblioteca Rust gera `rlib`, suficiente para o executável desktop; artefatos mobile não fazem parte desta prova. Em ambiente com pouco disco, os mesmos gates Rust podem usar `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0` para reduzir artefatos de depuração. A limpeza de `target/debug` remove apenas arquivos regeneráveis do projeto e deve terminar antes de iniciar nova compilação.

## Verificações

```sh
npm run build
npm run check:rust
npm run test:rust
npm run build:desktop
```

- `build`: checagem de tipos da interface/configuração e bundle Vite.
- `check:rust`: formatação sem escrita e Clippy com avisos tratados como erros; usa o lockfile.
- `test:rust`: testes de persistência/recuperação, limites de anexos, isolamento de eventos, estados terminais e exclusão mútua. Não comprova assinatura nem captura no jogo.
- `build:desktop`: gera o aplicativo `.app` local em `src-tauri/target/release/bundle/macos/Coplayer.app`. Executa também `verify:bundle` para conferir executáveis, identidade, versão mínima e hash do runtime. O script é voltado ao primeiro alvo macOS; outros formatos serão definidos na expansão de plataformas.

Abrir o aplicativo gerado e verificar `Núcleo conectado` e as abas das provas. Na 0.2, iniciar o runtime, conectar a assinatura e importar imagens; somente envio explícito inicia análise. Na 0.3, conceder a permissão quando solicitado, listar e escolher uma janela e testar captura/painel. Usar uma janela sintética antes do jogo permite validação preliminar sem declarar a 0.3 concluída. Instalação limpa e assinatura/notarização Apple permanecem posteriores.

O workflow `Checks` repete build, verificações Rust, executor de testes e bundle macOS. Sua execução remota depende de publicar o repositório no GitHub; a existência do arquivo não é um resultado de CI.

Na primeira instalação local, a resolução do crates.io parou de avançar com multiplexação HTTP. A execução foi retomada com `CARGO_HTTP_MULTIPLEXING=false` e `CARGO_HTTP_TIMEOUT=30` somente nos comandos afetados, sem alterar configurações globais. Aplicar esse ajuste apenas se o mesmo problema de rede ocorrer.

## Limites dos componentes

- `src/`: apresentação; `src/lib/` concentra chamadas nativas e tipos de retorno das provas.
- `src-tauri/src/provider/`: processo app-server, configuração consultiva, autenticação isolada e tradução de eventos.
- `src-tauri/src/platform/`: captura ScreenCaptureKit, foco AppKit, atalhos e coordenação de janelas. A ponte Objective-C é compilada dentro do aplicativo, sem processo auxiliar de captura.
- `src-tauri/src/storage/`: catálogo SQLite, migração, limites, integridade e arquivos privados.
- `src-tauri/src/`: entrada desktop e comandos Rust. Contratos consolidados em [Base local](core-contracts.md).
- `src-tauri/capabilities/`: permissões da WebView para ouvir eventos; não expõe shell ou filesystem genéricos ao frontend.
- `docs/`: especificação, planejamento e evidências. Regras de campanha e persistência futura permanecem em Rust, não nos componentes Vue.

O banco da 0.5 contém apenas anexos reais; campanhas e chats serão adicionados por migração na 1.1. Não criar interfaces vazias ou simulações de autenticação. O contrato inicial `AppInfo` contém `name`, `version`, `platform` e `architecture`; é diagnóstico da base, não estado de disponibilidade de IA.

Após substituir um bundle local, o macOS pode manter a permissão de Gravação da Tela vinculada ao executável anterior. Se os Ajustes mostram a permissão ativa mas o Coplayer não a reconhece, remova apenas a entrada Coplayer pela interface dos Ajustes, adicione o `.app` recém-gerado e escolha Encerrar e Reabrir. Autenticação do sistema é feita pelo jogador; não alterar TCC programaticamente.
