use serde::Serialize;
use serde_json::Value;

pub const MAX_RESPONSE_BYTES: usize = 256 * 1024;

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Analysis {
    pub request_id: String,
    pub thread_id: Option<String>,
    pub turn_id: Option<String>,
    pub status: String,
    pub text: String,
    pub message: String,
    pub model: String,
    pub image_count: usize,
    pub cancel_requested: bool,
    #[serde(skip)]
    pub requires_shutdown: bool,
    #[serde(skip)]
    pub final_text: Option<String>,
}

impl Analysis {
    pub fn active(&self) -> bool {
        matches!(self.status.as_str(), "starting" | "running" | "cancelling")
    }

    pub fn fail(&mut self, message: &str) {
        self.status = "failed".into();
        self.message = message.into();
    }

    // Notifications are accepted only for the current thread and turn. A new
    // thread is used for every proof, avoiding hidden conversational context.
    pub fn apply(&mut self, method: &str, params: &Value) {
        if !self.active()
            || params.get("threadId").and_then(Value::as_str) != self.thread_id.as_deref()
        {
            return;
        }
        let event_turn = params
            .get("turnId")
            .or_else(|| params.get("turn").and_then(|turn| turn.get("id")))
            .and_then(Value::as_str);
        let Some(event_turn) = event_turn else { return };
        if let Some(current) = self.turn_id.as_deref() {
            if current != event_turn {
                return;
            }
        } else {
            self.turn_id = Some(event_turn.into());
        }
        match method {
            "turn/started" => {
                if !self.cancel_requested { self.status = "running".into(); }
                self.message = "Análise em andamento pela assinatura.".into();
            }
            "item/agentMessage/delta" => {
                if let Some(delta) = params.get("delta").and_then(Value::as_str) {
                    if self.text.len().saturating_add(delta.len()) <= MAX_RESPONSE_BYTES {
                        self.text.push_str(delta);
                    } else {
                        self.requires_shutdown = true;
                        self.fail("Resposta excedeu o limite experimental. Encerre e reinicie o runtime.");
                    }
                }
            }
            "item/completed" => {
                let item = &params["item"];
                if item["type"].as_str() == Some("agentMessage")
                    && matches!(item["phase"].as_str(), None | Some("final_answer"))
                    && let Some(text) = item["text"].as_str()
                {
                    if text.len() <= MAX_RESPONSE_BYTES {
                        self.final_text = Some(text.into());
                        self.text = text.into();
                    } else {
                        self.requires_shutdown = true;
                        self.fail("Resposta final excedeu o limite experimental.");
                    }
                }
            }
            "turn/completed" => {
                match params.pointer("/turn/status").and_then(Value::as_str) {
                    Some("completed") if self.final_text.as_ref().is_some_and(|text| !text.trim().is_empty()) => {
                        self.status = "completed".into();
                        self.message = "Resposta concluída pela assinatura.".into();
                    }
                    Some("interrupted") => {
                        self.status = "cancelled".into();
                        self.message = "Análise interrompida; conteúdo parcial não é uma conclusão.".into();
                    }
                    _ => self.fail("A análise falhou. Verifique a conexão, a autenticação ou os limites da assinatura."),
                }
            }
            "error" => {
                self.message = "O provedor informou um erro; aguardando resultado terminal.".into();
            }
            _ => {}
        }
    }
}

pub fn is_chatgpt_account(value: &Value) -> bool {
    value.pointer("/account/type").and_then(Value::as_str) == Some("chatgpt")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn running() -> Analysis {
        Analysis {
            request_id: "proof-1".into(),
            thread_id: Some("thread-1".into()),
            turn_id: Some("turn-1".into()),
            status: "running".into(),
            ..Analysis::default()
        }
    }

    #[test]
    fn cancelled_and_failed_turns_are_not_success() {
        for (wire, expected) in [
            ("completed", "completed"),
            ("interrupted", "cancelled"),
            ("failed", "failed"),
            ("inProgress", "failed"),
        ] {
            let mut analysis = running();
            analysis.final_text = Some("Resposta completa".into());
            analysis.apply(
                "turn/completed",
                &json!({"threadId":"thread-1","turn":{"id":"turn-1","status":wire}}),
            );
            assert_eq!(analysis.status, expected);
        }
    }

    #[test]
    fn terminal_success_requires_a_complete_assistant_message() {
        let mut analysis = running();
        analysis.apply(
            "item/agentMessage/delta",
            &json!({"threadId":"thread-1","turnId":"turn-1","delta":"partial"}),
        );
        analysis.apply(
            "turn/completed",
            &json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed"}}),
        );
        assert_eq!(analysis.status, "failed");
        let mut analysis = running();
        analysis.apply("item/completed", &json!({"threadId":"thread-1","turnId":"turn-1","item":{"type":"agentMessage","phase":"final_answer","text":"Final"}}));
        analysis.apply(
            "turn/completed",
            &json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"completed"}}),
        );
        assert_eq!(analysis.status, "completed");
        assert_eq!(analysis.text, "Final");
    }

    #[test]
    fn stale_thread_and_turn_notifications_cannot_modify_analysis() {
        let mut analysis = running();
        for (thread, turn) in [("other", "turn-1"), ("thread-1", "other")] {
            analysis.apply(
                "item/agentMessage/delta",
                &json!({"threadId":thread,"turnId":turn,"delta":"stale"}),
            );
        }
        assert!(analysis.text.is_empty());
        assert_eq!(analysis.status, "running");
    }

    #[test]
    fn terminal_state_does_not_accept_late_output() {
        let mut analysis = running();
        analysis.apply(
            "turn/completed",
            &json!({"threadId":"thread-1","turn":{"id":"turn-1","status":"interrupted"}}),
        );
        analysis.apply(
            "item/agentMessage/delta",
            &json!({"threadId":"thread-1","turnId":"turn-1","delta":"late"}),
        );
        assert!(analysis.text.is_empty());
    }

    #[test]
    fn only_chatgpt_account_is_eligible() {
        assert!(is_chatgpt_account(&json!({"account":{"type":"chatgpt"}})));
        for value in [
            json!({"account":null}),
            json!({"account":{"type":"apiKey"}}),
            json!({"account":{"type":"amazonBedrock"}}),
        ] {
            assert!(!is_chatgpt_account(&value));
        }
    }

    #[test]
    fn response_memory_is_bounded() {
        let mut analysis = running();
        analysis.text = "a".repeat(MAX_RESPONSE_BYTES);
        analysis.apply(
            "item/agentMessage/delta",
            &json!({"threadId":"thread-1","turnId":"turn-1","delta":"x"}),
        );
        assert_eq!(analysis.status, "failed");
        assert_eq!(analysis.text.len(), MAX_RESPONSE_BYTES);
    }
}
