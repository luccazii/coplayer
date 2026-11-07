# Prova Legion Go S — subfase 0.4

Alvo confirmado pelo jogador: Legion Go S com SteamOS. Esta etapa prepara um **pacote experimental Linux x86_64**. Captura, overlay/ícone flutuante e retorno de foco ainda não estão implementados no Linux. O pacote permite começar pela janela Tauri, comunicação com Rust, entrada de texto, anexos importados e runtime de assinatura; nenhum desses resultados isolados conclui T05.

## Build pelo GitHub Actions

Workflow: `.github/workflows/legion.yml`, nome **Legion Linux package**. Usa runner `ubuntu-22.04` x86_64, Node de `.nvmrc`, Rust de `rust-toolchain.toml` e ambos os lockfiles. Executa build de frontend, Clippy/formatação, testes Rust, build AppImage e verificação de arquitetura/hashes/runtime. A versão do aplicativo continua 0.1.0; 0.4 é o identificador da subfase.

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

**Abrir o Coplayer pela biblioteca Steam não demonstra sobreposição sobre Victoria 3.** O pacote inicial não contém a integração Linux de captura/foco. A próxima implementação deve avaliar a sessão real antes de escolher esse caminho. Os critérios da 0.4 continuam:

| Capacidade | Evidência necessária | Estado inicial |
| --- | --- | --- |
| Janela e IPC | Interface nativa informa `linux / x86_64` | Pendente no aparelho |
| Controle e teclado virtual | Abrir painel, escrever e fechar sem enviar ações ao jogo | Pendente |
| Overlay sobre Victoria 3 | Painel aparece sobre o jogo no Gaming Mode | Não implementado |
| Retorno de foco | Fechar painel devolve controle ao jogo sem pausá-lo | Não implementado |
| Captura limpa | Imagem real do jogo sem superfícies do Coplayer; sem análise automática | Não implementado |
| Assinatura | Análise real concluída com imagens e cancelamento distinto de sucesso | Pendente no aparelho |

Evidência de impedimento deve identificar capacidade, ambiente e erro; não deve ser registrada como suporte. A prova no Legion Go S não valida Steam Deck.

## Referências de build

- [Tauri: AppImage e base Linux](https://v2.tauri.app/distribute/appimage/)
- [Tauri: dependências Linux](https://v2.tauri.app/start/prerequisites/)
- [Tauri: GitHub Actions](https://v2.tauri.app/distribute/pipelines/github/)
- [Runtime oficial fixado em 0.159.3](https://github.com/openai/codex/releases/tag/rust-v0.159.3)
