# Integrações reais com assinatura ChatGPT

> Pesquisa datada, preservada como evidência. Recomendações não são decisões definitivas de stack. Consulte [Produto e MVP](product-decisions.md) e [Pendências técnicas](technical-readiness.md) para o estado consolidado.

Pesquisa em 2026-11-07. Fontes primárias dos projetos e documentação oficial; nenhum projeto foi instalado ou executado.

## Evidências práticas

| Projeto | Evidência publicada | Aplicação ao Coplayer |
| --- | --- | --- |
| T3 Code | Documenta instalação gerenciada de Codex com compartilhamento do plano ChatGPT e, separadamente, uso de uma instalação com login Codex existente. | Frontend próprio e backend usando assinatura já são uma arquitetura praticada. |
| CodexMonitor | Aplicativo Tauri com app-server, anexos de imagens, filas e interrupção; também oferece backend remoto opcional. Exige CLI ou binário configurado. | Referência próxima do chat que precisamos, sem suas funcionalidades de programação. |
| Docker Agent | Documenta provedor ChatGPT com login OAuth, renovação e imagens, consumindo assinatura sem API Key. Implementa acesso ao backend Codex. | Outra evidência de integração, mas não implica que Docker/container seja obrigatório ou que devamos reproduzir seu acesso ao backend. |
| OpenHands | Exemplo de subscription_login com navegador ou device code e credenciais persistidas. Os nomes de modelos do exemplo precisam de atualização/validação. | Confirma existência de outra implementação; não é a recomendação inicial. |

Fontes:
- https://github.com/pingdotgg/t3code/blob/main/docs/user/providers-codex.md
- https://github.com/Dimillian/CodexMonitor
- https://docs.docker.com/ai/docker-agent/providers/chatgpt/
- https://github.com/OpenHands/software-agent-sdk/blob/main/examples/01_standalone_sdk/35_subscription_login.py

## Relato de operação

O issue T3 Code #16132, aberto em 2026-10-05 e atualmente fechado, relata conversas funcionando com assinatura, falhas posteriores à expiração do token e recuperação após reiniciar o aplicativo. É evidência de uso real e de uma falha a testar, não prova de defeito universal nem de que persiste nas versões atuais.

Fonte: https://github.com/pingdotgg/t3code/issues/16132

## Container

A documentação OpenAI inclui transferência de autenticação para um container Docker como alternativa para ambientes sem navegador, além do login por código de dispositivo. Isso comprova um caminho documentado para executar Codex autenticado em container, não a portabilidade do overlay ou autorização de serviço comercial hospedado.

Fonte: https://learn.chatgpt.com/docs/auth

## Recomendação arquitetural

Separar o overlay/captura nativos de um serviço local que mantém campanhas, fila e resumo e controla o runtime Codex. Usar SDK Python ou app-server diretamente é uma escolha de implementação desse serviço, não uma diferença no objetivo de consumir a assinatura.

Para o protótipo pessoal, priorizar processo auxiliar local; container fica opcional. Inferência de engenharia: container acrescenta gerenciamento do ambiente e transferência de capturas sem resolver por si só autenticação ou overlay. O runtime pode ser distribuído junto ao produto depois, sem obrigar instalação manual do Codex.

Autenticação deve permanecer em fluxo oficial. A documentação atual inclui projetos pessoais locais no Sign in with ChatGPT por registro dinâmico, sem homologação manual descrita. O app-server pode consumir a autorização desse fluxo; seu login gerenciado também existe, mas tem condições de uso próprias. A existência de um projeto de terceiros não equivale à homologação de todas as formas de distribuição.

Fontes:
- https://developers.openai.com/cookbook/articles/sign-in-with-chatgpt
- https://developers.openai.com/siwc/token-sharing-open-source/codex-app-server
- https://learn.chatgpt.com/docs/app-server#auth-endpoints

Conclusão: há evidência suficiente para propor frontend próprio + backend local + runtime Codex usando assinatura como caminho concreto de protótipo. Não é necessário resolver a distribuição comercial agora. Restam testes da conta, análise conjunta de capturas e reconexão, sem cobrança por API como fallback automático. A preferência por SDK já registrada permanece; não foi escolhida uma stack definitiva.
