//! Local campaign rules. No platform or provider dependencies.
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NewCampaign {
    pub name: String,
    pub game_version: String,
    pub active_dlcs: Vec<String>,
    pub dlcs_confirmed: bool,
    pub no_dlcs_confirmed: bool,
}
impl NewCampaign {
    pub fn validate(mut self) -> Result<Self, String> {
        self.name = bounded_text(
            &self.name,
            120,
            "Informe o nome da campanha (até 120 bytes).",
        )?;
        self.game_version = bounded_text(
            &self.game_version,
            80,
            "Informe a versão do jogo (até 80 bytes).",
        )?;
        if !self.dlcs_confirmed {
            return Err("Confirme as DLCs ativas desta campanha.".into());
        }
        if self.active_dlcs.len() > 64 {
            return Err("Informe no máximo 64 DLCs.".into());
        }
        let mut seen = std::collections::HashSet::new();
        for dlc in &mut self.active_dlcs {
            *dlc = bounded_text(dlc, 120, "Cada DLC precisa de um nome de até 120 bytes.")?;
            if !seen.insert(dlc.to_lowercase()) {
                return Err("O checklist contém DLCs repetidas.".into());
            }
        }
        if self.active_dlcs.is_empty() != self.no_dlcs_confirmed {
            return Err("Confirme explicitamente o jogo sem DLCs ou revise as DLCs ativas.".into());
        }
        Ok(self)
    }
}
pub fn bounded_text(value: &str, max: usize, message: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() || value.len() > max || value.chars().any(char::is_control) {
        return Err(message.into());
    }
    Ok(value.into())
}
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub id: String,
    pub campaign_id: String,
    pub game_version: String,
    pub active_dlcs: Vec<String>,
    pub dlcs_confirmed: bool,
    pub no_dlcs_confirmed: bool,
    pub source: String,
    pub created_at_ms: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub created_at_ms: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: String,
    pub campaign_id: String,
    pub configuration_id: String,
    pub title: String,
    pub draft: String,
    pub revision: i64,
    pub draft_generation: i64,
    pub created_at_ms: i64,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub campaigns: Vec<Campaign>,
    pub campaign_id: Option<String>,
    pub chat_id: Option<String>,
    pub chats: Vec<Chat>,
    pub configuration: Option<Configuration>,
    pub attachments: Vec<crate::storage::Attachment>,
    pub messages: Vec<LocalMessage>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DraftTarget {
    pub campaign_id: String,
    pub chat_id: String,
    pub generation: i64,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubmitMessage {
    pub target: DraftTarget,
    pub revision: i64,
    pub text: String,
    pub attachment_ids: Vec<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMessage {
    pub id: String,
    pub campaign_id: String,
    pub chat_id: String,
    pub configuration_id: String,
    pub text: String,
    pub status: String,
    pub created_at_ms: i64,
    pub attachment_count: u32,
}
