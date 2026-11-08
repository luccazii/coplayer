# Prova Legion Go S — subfase 0.4

Alvo confirmado pelo jogador: Legion Go S com SteamOS. Esta etapa prepara um **pacote experimental Linux x86_64**. Há um adaptador experimental X11/XWayland de captura isolada, painel/ícone e retorno de foco; Gaming Mode ainda não foi validado. O pacote permite começar pela janela Tauri, comunicação com Rust, entrada de texto, anexos importados e runtime de assinatura; nenhum desses resultados isolados conclui T05.

Pacote disponível: [v0.1.0-legion.1](https://github.com/luccazii/coplayer/releases/tag/v0.1.0-legion.1), com [download completo](https://github.com/luccazii/coplayer/releases/download/v0.1.0-legion.1/Coplayer-Legion-x86_64.tar.gz). Build Linux/macOS e 20 testes aprovados; funcionamento no aparelho permanece pendente.

## Build pelo GitHub Actions

Workflow: `.github/workflows/legion.yml`, nome **Legion Linux package**. Usa runner `ubuntu-22.04` x86_64, Node de `.nvmrc`, Rust de `rust-toolchain.toml` e ambos os lockfiles. Executa build de frontend, Clippy/formatação, testes Rust, build AppImage e verificação de arquitetura/hashes/runtime. A versão do aplicativo continua 0.1.0; 0.4 é o identificador da subfase.

O runtime oficial fica em `usr/libexec/Coplayer/runtime/codex`, configurado por `tauri.linux.conf.json`. O linuxdeploy usado pelo Tauri reescreve o RPATH dos arquivos ELF em `usr/lib`, incluindo esse runtime static PIE; `NO_STRIP` não impede essa alteração. `libexec` fica fora da varredura de bibliotecas. O núcleo resolve o caminho do pacote e a verificação extrai o AppImage final, recusa cópia duplicada em `usr/lib` e exige SHA-256 idêntico ao manifesto oficial. Nenhum hash alterado é aceito como substituto.

Após publicar o projeto no repositório escolhido, o workflow executa no push de `main` e `codex/legion-*`; também pode ser iniciado por **Actions → Legion Linux package → Run workflow** quando o arquivo estiver na branch padrão. O artefato `coplayer-legion-linux-x86_64-<commit>` só é publicado após os gates passarem e fica disponível por 14 dias. Não cria release pública e não precisa de credenciais ChatGPT na CI.

O ZIP contém `Coplayer-Linux-x86_64.AppImage`, `start-coplayer.sh`, este roteiro como `LEIA-ME.md` e `SHA256SUMS`. Ainda é necessário conferir a execução no SteamOS; compilar no Ubuntu não prova compatibilidade gráfica no aparelho.

O jogador autorizou o repositório público [luccazii/coplayer](https://github.com/luccazii/coplayer) e uma pré-release experimental. Após a CI aprovar o commit, o pacote poderá ser publicado em [Releases](https://github.com/luccazii/coplayer/releases) para download direto, sem depender do login exigido pelos artefatos do Actions. A publicação da pré-release é uma etapa explícita após conferir o run; o workflow não publica releases automaticamente.

## Build local alternativo com Docker

```sh
npm run build:legion
```

Usa `scripts/linux/Dockerfile` com `--platform linux/amd64`, inclusive em Mac ARM64. Saída em `artifacts/legion/`; `COPLAYER_ARTIFACT_DIR` permite escolher outro destino. Reserve 15–20 GiB para ferramentas e intermediários; o preparador recusa iniciar com menos de 12 GiB no volume do projeto. Confira também o espaço do Docker. A emulação pode ser lenta. O contexto é uma lista restrita de fontes; `.local`, autenticação, banco, capturas, runtime macOS e diretórios de build ficam fora. Docker/Node/Rust são ferramentas de build, não requisitos de execução no Legion.

## Primeiro teste no aparelho

1. No **Desktop Mode**, baixar o artefato do run bem-sucedido e extrair todos os arquivos para uma pasta permanente, por exemplo `~/Applications/Coplayer`.
2. Abrir o terminal nessa pasta e executar:

   ```sh
   sha256sum -c SHA256SUMS
   chmod +x Coplayer-Linux-x86_64.AppImage start-coplayer.sh
   ./start-coplayer.sh
   ```

   O launcher usa extração temporária do AppImage para dispensar montagem FUSE. Não exige `sudo`, alteração do sistema SteamOS nem instalação de ferramentas de desenvolvimento.
3. Conferir **Núcleo conectado · linux / x86_64** e o aviso de limitações Linux. Se a janela falhar, registrar o erro do terminal antes de tentar ajustes gráficos.
4. Testar clique/toque, rolagem, campo de pergunta e tamanho da janela. Importar apenas imagens sintéticas. Fechar e reabrir para conferir anexos persistidos, sem envio automático.
5. Clicar **Iniciar runtime**, depois conectar a assinatura no navegador. O login ocorre no aparelho, com dados próprios do Coplayer. Não copiar `auth.json` do Mac nem colocar credenciais no repositório/CI. A abertura do navegador depende de `xdg-open` e de uma sessão desktop funcional.
6. Enviar explicitamente as imagens de teste, conferir resposta terminal e cancelamento. Registrar falhas de navegador/runtime; o teste de versão do executável em CI não comprova autenticação.

## Gaming Mode: primeiro levantamento

Adicionar `start-coplayer.sh` como aplicativo não Steam e abrir pelo Gaming Mode. Manter o script e o AppImage juntos. Testar a janela como aplicativo, navegação por controle/toque e teclado virtual. Registrar versão do SteamOS, resolução/escala, método de entrada e resultado observado.

**Abrir o Coplayer pela biblioteca Steam não demonstra sobreposição sobre Victoria 3.** O adaptador atual usa Composite para capturar somente a superfície escolhida e EWMH para solicitar foco. Não há captura de desktop como alternativa silenciosa. Janelas Wayland nativas e outros servidores XWayland não aparecem na lista. O compositor pode negar captura, foco, sobreposição ou atalhos; esses resultados devem ser registrados por capacidade. Os critérios da 0.4 continuam:

| Capacidade | Evidência necessária | Estado inicial |
| --- | --- | --- |
| Janela e IPC | Interface nativa informa `linux / x86_64` | Pendente no aparelho |
| Controle e teclado virtual | Abrir painel, escrever e fechar sem enviar ações ao jogo | Pendente |
| Overlay sobre Victoria 3 | Painel aparece sobre o jogo no Gaming Mode | Implementação experimental; validação pendente |
| Retorno de foco | Fechar painel devolve controle ao jogo sem pausá-lo | Implementação experimental; validação pendente |
| Captura limpa | Imagem real do jogo sem superfícies do Coplayer; sem análise automática | Implementação experimental; validação pendente |
| Assinatura | Análise real concluída com imagens e cancelamento distinto de sucesso | Pendente no aparelho |

Evidência de impedimento deve identificar capacidade, ambiente e erro; não deve ser registrada como suporte. A prova no Legion Go S não valida Steam Deck.

## Referências de build

- [Tauri: AppImage e base Linux](https://v2.tauri.app/distribute/appimage/)
- [Tauri: dependências Linux](https://v2.tauri.app/start/prerequisites/)
- [Tauri: GitHub Actions](https://v2.tauri.app/distribute/pipelines/github/)
- [Runtime oficial fixado em 0.159.3](https://github.com/openai/codex/releases/tag/rust-v0.159.3)

## Diagnóstico local de sessão

Execute `bash scripts/linux/diagnose-session.sh` no Desktop Mode e novamente no Gaming Mode para comparar interfaces anunciadas. O script não captura a tela, não lista títulos de janelas e não acessa autenticação. Interfaces anunciadas não comprovam uma operação bem-sucedida.

Ferramentas já instaladas neste aparelho: `source .local/tools/env.sh` habilita Node/Rust. O contêiner persistente `coplayer-dev` foi criado com Ubuntu 22.04 (`distrobox enter coplayer-dev`); dependências nativas devem ser instaladas dentro dele, preservando o sistema SteamOS. Sua criação isolada não comprova build.


## Desenvolvimento neste SteamOS

Dependências nativas instaladas no Distrobox persistente `coplayer-dev` (Ubuntu 22.04), sem alterar a imagem do host. Node/Rust permanecem em `.local/tools`. Use:

```sh
scripts/linux/dev-container.sh npm run check:rust
scripts/linux/dev-container.sh npm run test:rust
scripts/linux/dev-container.sh npm run build:desktop:linux
```

Para reproduzir a preparação do contêiner: `distrobox create --name coplayer-dev --image docker.io/library/ubuntu:22.04 --yes`, seguido de instalação de `file build-essential pkg-config libwebkit2gtk-4.1-dev libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev patchelf libfuse2 squashfs-tools` via `sudo apt-get install` dentro dele.

O launcher configura `GDK_BACKEND=x11`. Atalhos Linux: `Ctrl+Shift+Espaço` (painel) e `Ctrl+Shift+9` (captura). Para a prova com controle, mapear esses atalhos e navegação de mouse pelo Steam Input; testar teclado virtual com Steam+X. Esses mapeamentos não comprovam isolamento de entrada do jogo. Se o foco não for confirmado em até um segundo, o painel reaparece com erro.

A captura aceita pixels RGB de 32 bits com máscaras RGB padrão, limita a superfície nativa a 32 Mi pixels, redimensiona aos limites do catálogo e preserva envio explícito. Formatos não suportados falham com mensagem. Referência da API: [x11rb Composite 0.13.2](https://docs.rs/x11rb/0.13.2/x11rb/protocol/composite/trait.ConnectionExt.html).

## Roteiro do pacote local 0.4

A pré-release `v0.1.0-legion.1` acima é anterior a este adaptador. Para testar o código novo, use o pacote local gerado nesta implementação em `artifacts/legion-0.4/`, depois de conferir `SHA256SUMS`.

1. Adicionar o `start-coplayer.sh` desse diretório como aplicativo não Steam. Manter AppImage e launcher juntos.
2. Em Gaming Mode, abrir Victoria 3 e Coplayer. Na aba **0.4 · Captura e painel**, atualizar a lista. Se o jogo não aparecer, registrar o impedimento; não capturar outra janela como substituto.
3. Selecionar o jogo, ativar o ícone e capturar. Conferir que a imagem é atual, pertence ao jogo e exclui as superfícies do Coplayer. Captura não deve iniciar análise.
4. Testar **Voltar ao aplicativo selecionado**, reabrir pelo ícone/atalho e repetir captura pelo atalho. Conferir foco e ausência de ações involuntárias no jogo.
5. Com Steam Input, testar os mesmos comandos pelo controle, tocar no campo de pergunta e abrir o teclado virtual com Steam+X. Registrar cada resultado separadamente.
6. Anotar SteamOS, resolução/escala, modo de janela do jogo, mapeamento do controle e erros exibidos. Só após isso classificar cada capacidade como viável ou impedida. Não enviar capturas pessoais ao repositório.

Validação local em 2026-11-08: build frontend, Clippy, 24 testes Rust e build AppImage aprovados. Teste nativo adicional de captura sintética passou. AppImage executado no host SteamOS sem Vite; anexos recuperados, nova captura e foco/painel por atalho verificados. Isso valida o caminho sintético KDE/XWayland, não Gaming Mode nem Victoria 3. Nenhuma nova análise pela assinatura foi enviada nesta validação.
