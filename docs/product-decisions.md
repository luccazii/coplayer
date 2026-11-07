# Produto e MVP

Consolidação das decisões da entrevista em 2026-11-07. Este documento define o produto aprovado; não escolhe tecnologias. Regras posteriores estão em [Evolução](roadmap.md), critérios em [Aceitação](acceptance-criteria.md) e questões em aberto em [Pendências técnicas](technical-readiness.md).

## Propósito e limites

O Coplayer ajuda a diagnosticar situações, comparar decisões e aprender Victoria 3. É consultivo: não pausa nem executa ações no jogo. A validação prioriza diagnóstico correto e identificação de lacunas; depois, utilidade dos próximos passos e suas consequências.

O escopo inicial cobre economia, política interna, diplomacia e guerra, incluindo jogo base e DLCs, sem mods. Usa o conhecimento do modelo; base curada e RAG não são requisitos do MVP. Reconhecer incertezas não garante ausência de erros confiantes. A avaliação deve distinguir leitura visual incorreta, contexto ausente e desconhecimento de regras.

A única observação automática do estado da partida vem de capturas explicitamente solicitadas. O jogador também pode informar contexto e registrar correções. Não há leitura de saves nem monitoramento visual contínuo.

## Entrega e plataformas

- MVP para uso pessoal, em macOS, inicialmente com jogo em janela sem bordas e janela comum como alternativa.
- Entrega gradual para Windows, Linux e SteamOS, incluindo Desktop e Gaming Mode do Steam Deck.
- Avaliar antecipadamente Gaming Mode, tela portátil e controle para evitar arquitetura incompatível; isso não exige entregar SteamOS junto ao MVP.
- Tela cheia nativa no macOS fica para avaliação posterior.
- Interface, respostas e leitura de telas devem ser validadas em português brasileiro e inglês. O idioma das respostas é independente do idioma do jogo.

## Fluxo 1 — Conectar e preparar uma campanha

OpenAI é o provedor inicial. Usar a assinatura pessoal ChatGPT é requisito para validar o protótipo, com praticidade e sem dependência de homologação manual do projeto. A conta e uma análise real pela assinatura ainda precisam ser validadas. Se esse acesso não funcionar, a validação do protótipo aguarda: API Key não substitui esse requisito.

API Key também é uma alternativa de uso, com cobrança separada e escolha explícita, inclusive ao atingir limites da assinatura. Nunca trocar silenciosamente para consumo pago. A captura e o contexto necessário são enviados ao provedor de IA com transparência.

O jogador cria ou seleciona manualmente uma campanha. Toda conversa pertence a uma campanha, inclusive perguntas gerais. Antes da primeira análise:

1. Informar o nome da campanha.
2. Informar a versão do jogo ou confirmar sua identificação por captura.
3. Confirmar o checklist de DLCs ativas. Todas começam desmarcadas; o jogador pode preencher manualmente ou confirmar/corrigir sugestões por captura. Continuar sem DLCs exige confirmação explícita.

País, data dentro do jogo e objetivos não são obrigatórios inicialmente. Quando faltar contexto necessário, o copiloto pergunta e salva a resposta com sua origem, sem tratar dados variáveis como permanentemente atuais. Ter uma DLC comprada não significa que esteja ativa.

O copiloto avisa quando observar possível incompatibilidade entre captura e campanha selecionada, sem trocar o histórico automaticamente. Essa verificação não garante detectar todas as trocas.

## Fluxo 2 — Definir objetivos e calibrar assistência

Objetivos são intenções estratégicas; metas são resultados verificáveis com indicador e prazo confirmados pelo jogador. Um objetivo pode existir sem metas. O copiloto pode propor metas, mas não estabelecê-las unilateralmente.

O jogador ordena objetivos por prioridade. Em conflitos relevantes, o copiloto explicita qual objetivo favorece, impactos e alternativas e consulta o jogador. Pode propor exceções justificadas à prioridade, sempre sujeitas à confirmação.

Sem objetivos, responde à dúvida e pede prioridades quando necessárias, sem inventar um objetivo padrão.

Configurações oferecem perfis Direto, Professor e Comparativo, com controles independentes de ensino, comparação e detalhamento. Preferências são globais e persistentes no MVP. Comparativo é o padrão, com explicações curtas e aprofundamento disponível. Compara alternativas quando há decisão; dúvidas factuais recebem resposta direta.

Pode explicar quais telas, menus e controles consultar, especialmente para ensinar. No MVP, instruções são textuais, sem marcações visuais. Distingue localizações confirmadas nas capturas de orientações incertas.

## Fluxo 3 — Abrir overlay e reunir capturas

O painel completo abre sob demanda por atalho de teclado ou ícone flutuante persistente e reposicionável. Ao abrir, recebe o foco de entrada; ao fechar, devolve-o ao jogo. Não pausa a partida.

Recomendações compactas podem permanecer fixadas, apenas visíveis enquanto o jogador interage com o jogo.

O jogador captura por atalho dedicado ou botão. O botão oculta temporariamente o overlay, captura o jogo e retorna ao chat. A imagem deve excluir painel, ícone e recomendações fixadas. Indicar o anexo e permitir removê-lo antes de enviar.

Uma mensagem pode reunir várias capturas. Capturar ou anexar não inicia análise: o envio explícito da mensagem inicia ou enfileira o pedido. A entrada é texto e imagens; respostas são escritas.

## Fluxo 4 — Analisar, esclarecer e comparar

Uma dúvida pode iniciar novo chat com contexto mais limpo. Recuperar os objetivos e somente o histórico relevante à pergunta, preservando origem e possível desatualização; não carregar integralmente todos os chats.

O copiloto distingue observações, inferências, informações fornecidas pelo jogador e lacunas. Com várias capturas, compara datas e informações visíveis e pede esclarecimento sobre diferenças relevantes. Pausar durante a coleta pode ser sugerido, nunca exigido ou executado. Capturas de momentos diferentes não são automaticamente um único estado.

O impacto de uma decisão depende dos recursos comprometidos, dificuldade de reversão e riscos aos objetivos no contexto da partida. Para baixo impacto, pode orientar provisoriamente com hipóteses explícitas. Para decisões importantes, pede contexto suficiente antes de recomendar. Se não puder avaliar o próprio impacto, também pede contexto.

O modo híbrido permite apontar situações relevantes não perguntadas pelo jogador, usando apenas capturas solicitadas. Não implica novas capturas automáticas ou conhecimento de mudanças posteriores à última imagem.

A interface mostra andamento e permite cancelar. Não há meta rígida inicial de latência; medir tempos reais priorizando qualidade.

### Fila

- Uma análise em execução por vez no aplicativo.
- Novas mensagens no mesmo chat aguardam na ordem de envio; outros chats também aguardam, sem cancelar a análise atual.
- Pode abrir chats, preparar e enfileirar mensagens enquanto espera.
- Diante de esclarecimento indispensável com fila pendente, o jogador escolhe responder agora ou continuar a fila.
- A resposta ao esclarecimento é vinculada à análise pendente, sem ir ao fim da fila. Continuar não autoriza inventar a informação faltante ou recomendar ações dependentes dela.
- A ordem é global por envio, preservando a ordem interna dos chats. Falha/cancelamento afeta apenas o pedido correspondente; falhas globais de autenticação pausam a fila. Reiniciar o aplicativo não reenvia pedidos automaticamente.
- “Responder agora” suspende o avanço da fila e mantém o esclarecimento vinculado à análise correta. “Continuar” libera os próximos pedidos, sem autorizar orientações dependentes da informação ausente.

## Fluxo 5 — Consultar resumo e seguir recomendações

O resumo da campanha fica em aba própria, com objetivos, prioridades, situação conhecida, decisões e dúvidas pendentes. Atualiza automaticamente após cada análise bem-sucedida, preservando origem e distinção entre informações recentes e antigas. Estar disponível não significa ser enviado inteiro em cada pergunta.

O jogador registra correções em observações e resumo sem exigir nova captura. A informação anterior fica marcada como corrigida, com vínculo à correção; as próximas análises usam a informação corrigida, identificada como fornecida pelo jogador.

Pode fixar e desafixar recomendações manualmente. Novas análises substituem automaticamente orientações desatualizadas, inclusive as fixadas, mantendo o histórico. Essa atualização não elimina confirmações de conflitos entre objetivos ou exceções às prioridades. Quando a substituição depende de confirmação, sinalizar a orientação como pendente, sem apresentar a nova ação como aprovada ou a anterior como atual. Recomendar não significa que a ação foi executada. Status de execução ficam para depois do MVP.

## Fluxo 6 — Retomar e atualizar a campanha

Campanhas, objetivos, chats, resumo e histórico ficam localmente no dispositivo. Não há sincronização automática no MVP. Manter informações anteriores com origem e possível desatualização, sem apresentá-las como atuais sem confirmação na nova captura.

Alterar versão ou DLCs atualiza a mesma campanha. Preservar a configuração de origem de observações e recomendações e reavaliar orientações dependentes das regras alteradas. Identificar uma versão não equivale a garantir conhecimento ou suporte validado a todas as versões.

Enquanto não houver ramificações, voltar a um save anterior exige outra campanha no Coplayer, com histórico vazio e opção de copiar objetivos; preservar a original.

Imagens ficam disponíveis localmente e podem ser excluídas manualmente no MVP. Retenção dinâmica será posterior. Após exclusão, manter informações extraídas, análises e referência à captura, sem prometer disponibilidade da imagem para consulta ou reenvio. Retenção local não define a política do provedor remoto.

## Stack consolidada

A base escolhida é Tauri 2, Rust, Vue 3, TypeScript, SQLite e capturas em arquivos locais. O caminho inicial de IA usa Codex app-server diretamente, com runtime empacotado, substituindo a preferência anterior por avaliar SDK Python. Não exigir instalação manual separada do Codex nem Docker para executar o aplicativo.

A [stack](stack.md) registra responsabilidades e justificativa. Captura, foco, Gaming Mode, autenticação pela assinatura e distribuição continuam sujeitos às provas de [preparação técnica](technical-readiness.md). A base de desenvolvimento 0.1 foi iniciada após aprovação do [plano por subfases](implementation-plan.md); isso não comprova suporte às funcionalidades do MVP.
