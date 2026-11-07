# Investigação do Codex SDK

> Pesquisa datada, preservada como evidência. Recomendações não são decisões definitivas de stack. Consulte [Produto e MVP](product-decisions.md) e [Pendências técnicas](technical-readiness.md) para o estado consolidado.

Data: 2026-11-07. Pesquisa documental e leitura do código oficial; sem instalação, login, inferência ou implementação. Os arquivos de `main` são evidência do desenvolvimento atual, não garantia de conteúdo de toda versão publicada.

## Conclusão

O SDK é um candidato tecnicamente adequado para avaliar o MVP pessoal local. Ambos os SDKs aceitam entradas com várias imagens. Python oferece a superfície mais completa para autenticação embutida e controle de conversas; TypeScript é uma camada mais fina sobre o CLI. A escolha da linguagem não foi aprovada nem implementada.

## Runtime e instalação

### Python

O pacote `openai-codex` instala automaticamente uma versão compatível de `openai-codex-cli-bin`. O runtime é distribuído como pacote binário específico por plataforma. Não exige que o usuário instale previamente o aplicativo Codex. Um instalador final do Coplayer ainda terá de incluir Python, bibliotecas e runtime: instalação via pip para desenvolvimento não produz automaticamente um aplicativo desktop autossuficiente.

Fontes:
- https://github.com/openai/codex/blob/main/sdk/python/docs/getting-started.md
- https://github.com/openai/codex/blob/main/sdk/python-runtime/README.md
- https://learn.chatgpt.com/docs/codex-sdk

### TypeScript

O SDK executa o CLI e recebe eventos JSONL. O resolvedor procura `@openai/codex` e seus pacotes nativos; também aceita um caminho explícito para o executável. Há alvos macOS, Windows e Linux para x64 e arm64. Isso permite empacotar o executável no Coplayer, mas não comprova que qualquer instalação do SDK isoladamente contém todos os binários. A consulta ao pacote npm foi bloqueada com HTTP 403; a composição de uma versão publicada não foi verificada. Não usar o manifest de desenvolvimento como prova da composição do pacote publicado.

Fontes:
- https://github.com/openai/codex/blob/main/sdk/typescript/src/exec.ts
- https://github.com/openai/codex/blob/main/sdk/typescript/src/codexOptions.ts
- https://github.com/openai/codex/blob/main/sdk/typescript/package.json

## Autenticação pela assinatura

Python fornece login ChatGPT pelo navegador, login por código de dispositivo, consulta de conta e logout; aceita API Key e reutiliza autenticação existente. No Coplayer, preferir armazenamento e configuração próprios para não interferir nas sessões de desenvolvimento do usuário.

Fonte: https://github.com/openai/codex/blob/main/sdk/python/README.md

A classe principal TypeScript inspecionada oferece criação e retomada de threads, mas não métodos de login. Precisaríamos integrar o login do executável ou outra camada de autenticação; instalar o SDK não autentica automaticamente uma conta nova.

Fonte: https://github.com/openai/codex/blob/main/sdk/typescript/src/codex.ts

Suporte técnico ao login não é homologação de um produto. O SDK Python utiliza o app-server e mantém as condições dessa modalidade. O caminho explicitamente documentado para novos projetos pessoais locais sem avaliação manual continua sendo Sign in with ChatGPT com registro dinâmico; combinar esse fluxo com o runtime/SDK precisa de validação específica. Não prometer que o SDK concede uma exceção às regras de acesso.

Fontes:
- https://learn.chatgpt.com/docs/app-server#auth-endpoints
- https://developers.openai.com/cookbook/articles/sign-in-with-chatgpt
- https://developers.openai.com/siwc/token-sharing-open-source/codex-app-server

## Imagens e fluxo de chat

O exemplo oficial TypeScript envia texto e duas imagens locais no mesmo `run()`. A API Python aceita uma lista de entradas, incluindo `TextInput` e `LocalImageInput`. Assim, o envio conjunto de capturas é suportado no contrato; ainda não testamos leitura, limites ou qualidade com Victoria 3.

Fontes:
- https://github.com/openai/codex/blob/main/sdk/typescript/README.md#attaching-images
- https://github.com/openai/codex/blob/main/sdk/python/docs/api-reference.md#inputs

Python também expõe retomada, instruções por thread, streaming e interrupção. Isso cobre os mecanismos para contexto selecionado, progresso e cancelamento. Filas, resumo da campanha, decisões, correções e retenção continuam sendo responsabilidades do Coplayer. Não assumir que a persistência nativa do Codex cumpre a exclusão de imagens do produto.

Fonte: https://github.com/openai/codex/blob/main/sdk/python/docs/api-reference.md

## Recomendação e pendências

Recomendação técnica: avaliar o SDK Python como serviço auxiliar local, mantendo a interface desacoplada; se a stack escolhida for integralmente TypeScript, comparar o custo de adicionar Python com integrar app-server diretamente. SDK Python e app-server não são alternativas excludentes: o primeiro controla o segundo.

Antes de considerar viável a entrega:

1. Escolher uma versão publicada e verificar seus artefatos para o Mac alvo.
2. Validar autenticação e uma resposta terminal concluída com múltiplas imagens, usando a assinatura e sem fallback silencioso para API.
3. Configurar o runtime para assistência consultiva: impedir ações no jogo e ferramentas desnecessárias; modo de arquivos somente leitura, isoladamente, não comprova ausência de ações externas.
4. Verificar armazenamento próprio, imagens persistidas pelo runtime e limpeza compatível com o produto.
5. Medir latência e testar cancelamento e retomada.
6. Validar o instalador em máquina sem Codex e sem ambiente de desenvolvimento pré-instalado.
7. Testar overlay e captura por plataforma. Binário Linux não comprova suporte ao Gaming Mode.

Esses testes ainda não foram executados. Esta investigação não altera o escopo de apenas definição e documentação.
