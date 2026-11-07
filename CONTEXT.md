# Coplayer

Copiloto consultivo para jogos de estratégia que ajuda o jogador a compreender a partida e decidir conforme suas intenções. Este glossário inclui conceitos da evolução futura; o escopo de entrega está em [Produto e MVP](docs/product-decisions.md).

## Language

### Partida e conversas

**Copiloto**:
Assistente consultivo que orienta o jogador sem executar ações no jogo nem pausá-lo.

**Campanha**:
Partida contínua identificada pelo jogador, com contexto e histórico próprios entre períodos de jogo.
_Avoid_: Sessão como sinônimo de campanha.

**Ramificação**:
Caminho alternativo de uma campanha retomado de um ponto anterior confirmado pelo jogador.

**Sessão de chat**:
Conversa sobre uma dúvida, vinculada a uma campanha e, quando aplicável, a uma ramificação.
_Avoid_: Sessão de jogo como sinônimo de sessão de chat.

**Dia ativo**:
Data local com envio de uma captura nova identificada como pertencente à campanha selecionada.
_Avoid_: Dia corrido, dia dentro do jogo.

**Histórico da partida**:
Registro das observações, informações fornecidas, análises, decisões e correções de um caminho da campanha, com suas origens.

**Resumo da campanha**:
Síntese consultável de objetivos, prioridades, situação conhecida, decisões e dúvidas pendentes de um caminho da campanha.

### Intenções e orientação

**Objetivo**:
Intenção estratégica do jogador que orienta recomendações, sem exigir indicador ou prazo.
_Avoid_: Meta como sinônimo de objetivo.

**Meta**:
Resultado verificável associado a um objetivo, com indicador e prazo confirmados pelo jogador.
_Avoid_: Objetivo como sinônimo de meta.

**Prioridade**:
Ordem de preferência entre objetivos definida pelo jogador.

**Conflito entre objetivos**:
Situação em que uma alternativa favorece um objetivo e prejudica outro.

**Impacto de uma decisão**:
Magnitude das consequências de uma escolha, considerando recursos comprometidos, dificuldade de reversão e riscos aos objetivos.

**Recomendação**:
Orientação de ação proposta ao jogador a partir do contexto conhecido.
_Avoid_: Ação executada como sinônimo de recomendação.

**Análise sob demanda**:
Avaliação do contexto conhecido em resposta a uma solicitação do jogador, considerando seus objetivos quando definidos.

**Alerta proativo**:
Aviso iniciado pelo copiloto sobre uma situação relevante identificada nas capturas já solicitadas pelo jogador.

### Evidência e conhecimento

**Captura**:
Registro visual da tela obtido por comando explícito do jogador.

**Observação**:
Informação identificada diretamente em uma captura da partida.

**Inferência**:
Conclusão derivada do contexto conhecido, sem confirmação direta na tela.

**Lacuna de informação**:
Informação necessária à análise que ainda não está disponível no contexto conhecido.

**Correção**:
Informação fornecida pelo jogador que substitui um registro incorreto para análises futuras, preservando sua origem.

## Relationships

- Uma **Campanha** tem zero ou mais **Objetivos** e **Sessões de chat**.
- Um **Objetivo** tem zero ou mais **Metas**; cada **Meta** pertence a um **Objetivo**.
- A **Prioridade** ordena **Objetivos**; um **Conflito entre objetivos** envolve pelo menos dois deles.
- Uma **Campanha** pode ter múltiplas **Ramificações**, que compartilham histórico até o ponto de retorno e mantêm caminhos distintos depois dele.
- Uma **Sessão de chat** pertence a uma **Campanha** e, quando aplicável, a uma **Ramificação**.
- Cada caminho tem seu **Histórico da partida** e **Resumo da campanha**.
- Toda **Observação** tem uma **Captura** de origem; informação fornecida pelo jogador tem origem distinta da observação visual.
- Uma **Análise sob demanda** usa o contexto conhecido e pode produzir **Recomendações**, **Inferências** e **Lacunas de informação**.
- Uma **Correção** referencia o registro anterior e identifica o jogador como sua fonte.
- Um **Alerta proativo** usa evidência já capturada; não implica observação contínua.

## Example dialogue

> **Jogador:** Meu objetivo é industrializar o país, mas ainda não defini uma meta.
> **Copiloto:** Posso propor um indicador e um prazo para você confirmar. Nesta captura observei um dado que pode afetar esse objetivo.
> **Jogador:** Esse número foi lido errado; quero corrigi-lo.
> **Copiloto:** A correção fica vinculada à observação anterior. Podemos usá-la nesta sessão de chat sem apagar o histórico da campanha.

## Flagged ambiguities

- Intenção é **Objetivo**; resultado verificável com indicador e prazo é **Meta**.
- **Sessão de chat** não significa abrir o jogo nem entrar no save.
- **Dia ativo** é data local de atividade evidenciada, não uma data do calendário da partida.
- O **Histórico da partida** contém mais do que observações visuais; declarações e correções têm origem própria.
- **Alerta proativo** não significa captura automática ou conhecimento em tempo real.
