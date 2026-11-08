# Critérios de aceitação do MVP

Derivados das decisões aprovadas. **T01 verificado em 2026-11-07** no bundle macOS ARM64, com assinatura ChatGPT e duas imagens sintéticas; evidências no [registro de execução](implementation-plan.md). **C01/C02 verificados no cadastro manual da 1.1**: núcleo exige nome, versão e confirmação de DLCs, inclusive confirmação explícita de lista vazia. A barreira no envio integrado deverá ser revalidada na 1.3; sugestão visual não foi implementada. Os demais critérios completos permanecem **não verificados**, ainda que tenham provas parciais registradas. Os cenários são independentes da stack; comportamentos posteriores do [roadmap](roadmap.md) não bloqueiam o MVP.

## Pré-requisitos técnicos

| ID | Cenário | Resultado esperado |
| --- | --- | --- |
| T01 | Conectar uma conta elegível e enviar texto com múltiplas imagens | Resposta terminal bem-sucedida usando assinatura, sem consumo de API como substituto silencioso |
| T02 | Conectar via API Key por escolha explícita | Modalidade de cobrança separada identificada; assinatura continua requisito prévio do protótipo |
| T03 | Atingir limite da assinatura ou perder autenticação | Informar indisponibilidade/necessidade de reconexão e oferecer alternativa explícita; não declarar resposta incompleta como sucesso |
| T04 | Instalar no Mac alvo sem aplicativo Codex previamente instalado | Runtime necessário acompanha o produto e o fluxo de conexão funciona; não exigir instalação manual separada do Codex |
| T05 | Avaliar Gaming Mode antes de fechar arquitetura | Produzir evidência de viabilidade ou impedimentos de overlay, captura, foco e controle; não declarar suporte entregue com base apenas em binário Linux |

## Campanha e contexto

| ID | Cenário | Resultado esperado |
| --- | --- | --- |
| C01 | Criar campanha | Nome, versão e confirmação de DLCs exigidos antes da primeira análise; país, data e objetivos opcionais |
| C02 | Deixar DLCs todas desmarcadas | Exigir confirmação explícita de jogo sem DLCs; permitir edição manual ou sugestão por captura confirmada |
| C03 | Fazer pergunta sem país definido quando ele for necessário | Pedir o país e salvar a resposta com origem, sem inventá-lo |
| C04 | Abrir outro chat da campanha | Recuperar objetivos e histórico relevante; preservar origem e desatualização sem enviar todas as conversas |
| C05 | Selecionar outra campanha | Não misturar contexto; avisar sobre incompatibilidades visuais reconhecidas sem trocar seleção automaticamente |
| C06 | Atualizar versão ou DLCs | Manter campanha e configurações históricas de origem; reavaliar recomendações afetadas |
| C07 | Voltar a save anterior | Usar outra campanha com histórico vazio e cópia opcional de objetivos; original preservada |

## Overlay, captura e entrada

**Prova parcial da 1.2:** no aplicativo nativo macOS, janela sintética validou captura limpa por botão/atalho, alternância de foco confirmada pelo jogador, remoção, isolamento por chat, registro local explícito e recuperação de texto/imagens após reinício. Escopo sintético escolhido pelo jogador; regressão no Victoria 3 e envio integrado pela assinatura permanecem pendentes. Isso não conclui O01–O03 completos.

| ID | Cenário | Resultado esperado |
| --- | --- | --- |
| O01 | Abrir pelo atalho ou ícone reposicionável em janela sem bordas/janela comum | Painel recebe foco e fecha devolvendo-o ao jogo, sem pausar |
| O02 | Capturar pelo botão e pelo atalho | Anexar imagem e indicar sucesso, sem painel, ícone ou recomendações fixadas na captura |
| O03 | Reunir e remover anexos | Nenhuma análise automática ao capturar; envio explícito reúne as imagens restantes com a pergunta |
| O04 | Fixar e desafixar orientação | Mostrar conteúdo compacto durante o jogo; abrir o painel para interagir sem enviar comandos do chat ao jogo |
| O05 | Jogo em inglês e respostas em português brasileiro, e demais combinações dos dois idiomas | Respeitar idioma escolhido e avaliar leitura das telas; interface disponível nos dois idiomas |

## Análise e assistência

| ID | Cenário | Resultado esperado |
| --- | --- | --- |
| A01 | Capturas de momentos diferentes com divergência reconhecível | Explicitar a divergência e pedir esclarecimento quando relevante; não combinar silenciosamente como um único estado |
| A02 | Dúvida factual versus escolha estratégica no perfil padrão | Resposta direta à primeira; alternativas, consequências e relação com objetivos na segunda |
| A03 | Ajustar ensino, comparação e detalhamento | Aplicar controles personalizáveis dos perfis; explicações curtas por padrão e aprofundamento disponível |
| A04 | Conflito relevante entre objetivos ou exceção à prioridade | Apresentar prioridade favorecida e impactos, aguardando decisão do jogador |
| A05 | Propor meta | Exigir confirmação de indicador e prazo; objetivo pode continuar sem metas |
| A06 | Informação insuficiente para ação de alto impacto ou impacto desconhecido | Pedir contexto antes de recomendar; orientação provisória de baixo impacto identifica hipóteses |
| A07 | Informação antiga ou corrigida no histórico | Não tratar observação antiga como atual sem confirmação; usar correção identificada como fornecida pelo jogador |
| A08 | Solicitar orientação de interface | Indicar telas/controles em texto e explicitar localização não confirmada, sem clicar ou pausar |
| A09 | Detectar problema não perguntado | Pode alertar a partir das capturas solicitadas, sem alegar observação contínua |

## Conversas, persistência e recomendações

| ID | Cenário | Resultado esperado |
| --- | --- | --- |
| H01 | Enviar mensagens durante uma análise, no mesmo chat e em outros | Uma execução global por vez; ordem no mesmo chat preservada; permitir preparar/enfileirar sem cancelar a atual |
| H02 | Solicitar esclarecimento com fila pendente | Oferecer responder agora ou continuar; resposta ao esclarecimento vinculada à análise correta |
| H03 | Cancelar uma análise | Distinguir cancelamento de sucesso; afetar somente o pedido correspondente, preservando a fila global por envio; após reinício não reenviar automaticamente |
| H04 | Concluir análise e consultar resumo | Atualização automática com objetivos, prioridades, situação, decisões, dúvidas e origens |
| H05 | Corrigir observação ou resumo | Preservar registro anterior marcado como corrigido e usar a nova informação nas próximas análises |
| H06 | Nova análise tornar orientação fixada desatualizada | Substituir automaticamente e preservar histórico, sem contornar confirmação de conflito/prioridade |
| H07 | Fechar e reabrir o produto | Recuperar dados locais da campanha; não depender de sincronização externa |
| H08 | Excluir imagem manualmente | Remover sua disponibilidade para revisão/reenvio; preservar informação extraída e referência histórica; verificar cópias do runtime |

## Avaliação qualitativa

Validar com situações reais de economia, política interna, diplomacia e guerra. Registrar capturas/contexto de entrada, diagnóstico esperado, lacunas identificadas, resposta produzida, erro observado e tempo de resposta. Priorizar correção do diagnóstico antes da qualidade das recomendações.

O plano define 16 casos iniciais: quatro áreas do jogo × dois idiomas de tela × dois idiomas de resposta. Conteúdo dos casos, versões/DLCs de teste e limiar de qualidade serão fechados antes da execução de 3.2. Não declarar precisão geral, cobertura de todas as versões ou prazo de resposta garantido com base em poucos exemplos. A avaliação de latência é descritiva inicialmente, sem limite rígido aprovado.
