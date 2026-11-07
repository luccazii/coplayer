# Viabilidade da integração com assinatura ChatGPT

> Pesquisa datada, preservada como evidência. Recomendações não são decisões definitivas de stack. Consulte [Produto e MVP](product-decisions.md) e [Pendências técnicas](technical-readiness.md) para o estado consolidado.

Consulta documental: 2026-11-07. Nenhuma integração implementada, autenticação iniciada ou inferência executada.

## Conclusão

Para o MVP pessoal e local, recomendamos Sign in with ChatGPT com registro dinâmico e acesso direto à Responses API. A pesquisa complementar encontrou inclusão explícita de projetos pessoais locais no cookbook oficial: não é necessário decidir pela abertura do código para este MVP. O caminho documentado não exige homologação manual do projeto; consentimento, elegibilidade da conta e sucesso da inferência ainda precisam de teste real.

Fonte decisiva: https://developers.openai.com/cookbook/articles/sign-in-with-chatgpt (publicado em 2026-09-28, consultado em 2026-11-07).

Essa conclusão substitui a dúvida anterior sobre precisar publicar o código ou esclarecer previamente o uso pessoal local. Não se estende automaticamente a distribuição comercial ou hospedagem remota.

## Caminhos documentados

### Sign in with ChatGPT

A documentação descreve autorização para usar o plano em requisições elegíveis à Responses API, para aplicativos open-source e locais. Aplicativos pagos ou hospedados remotamente são encaminhados a um formulário de interesse. Isso não permite presumir aprovação de uma futura versão comercial nem obriga publicar agora o repositório.

Fonte: https://developers.openai.com/siwc/token-sharing-open-source

O registro dinâmico usa o nome real do aplicativo, identificador persistente da instalação, consentimento no navegador e callback local com PKCE. Não exige segredo de cliente ou chave de API de parceiro. O cliente emitido é associado à conta e ao workspace autorizados.

Fonte: https://developers.openai.com/siwc/token-sharing-open-source/sign-in

A página geral de solicitação de client ID descreve acesso para parceiros comerciais selecionados e lista de espera. Deve ser distinguida do fluxo de registro dinâmico documentado para OSS, sem concluir que todo aplicativo local precisa dessa lista nem que todo aplicativo é elegível.

Fonte: https://developers.openai.com/siwc/request-client-id

### Codex app-server

O app-server oferece autenticação ChatGPT gerenciada e entradas de texto e imagens locais. A documentação permite continuidade de aplicações locais ou open-source existentes, recomenda migração para Sign in with ChatGPT e exclui serviços comerciais ou hospedados dessa autenticação. Para um projeto novo, recomendamos avaliar primeiro o fluxo dedicado de Sign in with ChatGPT, sem assumir que a autenticação antiga elimina os requisitos de elegibilidade.

Fonte: https://learn.chatgpt.com/docs/app-server

O app-server também pode operar com um token obtido pelo novo fluxo; portanto, usar assinatura não obriga escolher o app-server como arquitetura do Coplayer.

Fonte: https://developers.openai.com/siwc/token-sharing-open-source/codex-app-server

## Compatibilidade com o produto

- Texto e imagens são aceitos quando suportados pelo modelo selecionado. Áudio e transcrição não são suportados nesse fluxo, coerente com a decisão inicial de chat escrito.
- O fluxo exige streaming e `store: false`; o histórico necessário deve ser reenviado. Isso combina com o histórico local planejado, mas não é garantia de retenção zero pelo provedor.
- File search hospedado não é suportado nesse fluxo; o futuro RAG precisará de desenho próprio compatível.

Fonte: https://developers.openai.com/siwc/token-sharing-open-source/preview-limitations

- Os modelos devem ser consultados para a conta autenticada; não presumir acesso a um modelo específico.
- Apenas uma resposta terminal concluída comprova sucesso. Limites da assinatura podem gerar erro mesmo após começar a transmitir texto.

Fonte: https://developers.openai.com/siwc/token-sharing-open-source/models-and-inference

## Próximas verificações, ainda não executadas

1. Manter a validação no escopo pessoal e local escolhido. Reavaliar requisitos antes de eventual distribuição comercial ou hospedagem remota; abertura do código não é pré-requisito do MVP pessoal local.
2. Validar, em trabalho posterior autorizado, registro e consentimento na conta do jogador, sem reutilizar credenciais internas de outro aplicativo.
3. Completar uma análise com múltiplas imagens usando exclusivamente a assinatura e verificar limites, renovação de autenticação e tratamento de falhas.
4. Avaliar qualidade e latência em capturas reais de Victoria 3; documentação de API não comprova qualidade de diagnóstico.
5. Validar o fluxo de autenticação e o overlay em cada plataforma. O protocolo documentado não comprova funcionamento no SteamOS Gaming Mode.

Nenhum dos itens acima foi executado nesta investigação documental. A alternativa por API Key não substitui o requisito de assinatura decidido pelo jogador.
