# Evolução após o MVP

As regras abaixo preservam decisões aprovadas, sem ampliar os critérios da primeira entrega. Não há datas nem ordem aprovada entre todos os itens.

## Etapa seguinte: ramificações e retenção dinâmica

### Ramificações

Retomar um save anterior cria um caminho separado na mesma campanha, preservando o original. O jogador informa ou confirma o ponto de retorno; o copiloto não o identifica com garantia apenas por imagens.

Compartilhar somente o histórico anterior ao ponto de retorno, sem acontecimentos posteriores de outro caminho. Cada ramificação mantém seu resumo e contexto. Ao criar, escolher objetivos, metas e prioridades do ponto de retorno ou do plano mais recente, com revisão; o padrão é o plano do ponto de retorno.

### Retenção dinâmica de imagens

Excluir conforme relevância e uso, não por prazo fixo de calendário. Informações extraídas, análises e referência de origem permanecem após excluir a imagem.

Uma imagem só fica elegível à avaliação de exclusão quando cumprir conjuntamente, desde seu último uso em análise concluída com sucesso:

- **Quatro sessões de chat posteriores válidas**: cada chat conta uma vez, após a primeira análise bem-sucedida; vazios e tentativas falhas não contam. O chat do uso da imagem não conta como sessão posterior.
- **Dois dias ativos posteriores**, em datas locais distintas: o dia do último uso não conta. Um dia ativo exige nova captura enviada e identificada com a campanha selecionada. Não há confirmação manual alternativa; abrir o overlay ou conversar sem captura não conta.

As contagens são globais: qualquer campanha ou ramificação pode contribuir. Uma data conta uma única vez, mesmo com várias sessões/campanhas. Dias sem atividade não contam. Reutilizar a imagem em análise bem-sucedida reinicia ambas as contagens. Não há terceiro contador por quantidade de análises.

Expiração não obriga exclusão. Depois da proteção, excluir se substituída por evidência recente ou sem utilidade para questões pendentes; preservar se ainda necessária, inclusive em campanha inativa. O escopo global dos contadores não mistura históricos.

Exemplo: usada na segunda-feira, a imagem continua protegida se houver quatro chats válidos somente na terça, pois ocorreu apenas um dia ativo posterior. Se quarta também for dia ativo, os dois mínimos podem estar cumpridos; ainda é necessário avaliar relevância. Reuso reinicia a contagem.

O algoritmo de relevância, a contagem de chats antigos retomados e detalhes de exclusão de cópias internas do runtime continuam pendências técnicas; não presumir comportamento já escolhido para esses casos.

## Outras evoluções aprovadas

- Acompanhamento de recomendações: jogador marca realizada, ignorada ou adiada. Capturas podem sugerir atualização de status, mas só confirmação do jogador a efetiva.
- Windows, Linux e SteamOS Desktop/Gaming Mode, com navegação por controle e interface adequada à tela portátil. Ordem após macOS ainda não definida.
- RAG com documentação de cada jogo.
- Abertura a outros provedores de LLM, com integração validada para cada um.

## Possibilidades para avaliação futura

Não são compromissos de entrega definidos:

- Sincronização entre dispositivos.
- Voz, mediante solução de transcrição adequada.
- Marcações visuais no jogo ou nas capturas.
- Leitura de saves e outras fontes estruturadas.
- Tela cheia nativa no macOS.
- Distribuição a convidados ou público geral, modelo comercial e licença do projeto.

Mods estão fora do escopo inicial e não têm suporte futuro aprovado.
