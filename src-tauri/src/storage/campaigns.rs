use super::*;
use crate::campaign::{Campaign, Chat, Configuration, NewCampaign, Workspace, bounded_text};

pub(super) fn entity_id(kind: &str) -> String {
    identifier().replacen("attachment", kind, 1)
}
pub(super) fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}
impl Store {
    pub fn workspace(&self) -> Result<Workspace, String> {
        let mut statement = self
            .connection
            .prepare("SELECT id, name, created_at_ms FROM campaigns ORDER BY created_at_ms, id")
            .map_err(failure)?;
        let campaigns = statement
            .query_map([], |row| {
                Ok(Campaign {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    created_at_ms: row.get(2)?,
                })
            })
            .map_err(failure)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(failure)?;
        let selection: Option<(String, String)> = self
            .connection
            .query_row(
                "SELECT campaign_id, chat_id FROM local_selection WHERE singleton=1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(failure)?;
        let (campaign_id, chat_id) = selection
            .map(|(a, b)| (Some(a), Some(b)))
            .unwrap_or_default();
        let mut chats = Vec::new();
        let mut configuration = None;
        if let (Some(campaign), Some(chat)) = (&campaign_id, &chat_id) {
            let mut statement = self.connection.prepare("SELECT id, campaign_id, configuration_id, title, draft, revision, created_at_ms, draft_generation FROM chats WHERE campaign_id=?1 ORDER BY created_at_ms, id").map_err(failure)?;
            chats = statement
                .query_map([campaign], |row| {
                    Ok(Chat {
                        id: row.get(0)?,
                        campaign_id: row.get(1)?,
                        configuration_id: row.get(2)?,
                        title: row.get(3)?,
                        draft: row.get(4)?,
                        revision: row.get(5)?,
                        created_at_ms: row.get(6)?,
                        draft_generation: row.get(7)?,
                    })
                })
                .map_err(failure)?
                .collect::<Result<Vec<_>, _>>()
                .map_err(failure)?;
            configuration = Some(self.analysis_context(campaign, chat)?);
        }
        let attachments = match chat_id.as_deref() {
            Some(id) => self.previews(self.scoped_rows(None, Some(id), None)?)?,
            None => vec![],
        };
        let messages = match chat_id.as_deref() {
            Some(id) => self.messages(id)?,
            None => vec![],
        };
        Ok(Workspace {
            campaigns,
            campaign_id,
            chat_id,
            chats,
            configuration,
            attachments,
            messages,
        })
    }
    /// Resolve an explicit owned chat, never infer it from the current UI selection.
    pub fn analysis_context(
        &self,
        campaign_id: &str,
        chat_id: &str,
    ) -> Result<Configuration, String> {
        let raw = self.connection.query_row("SELECT c.id, c.campaign_id, c.game_version, c.active_dlcs, c.dlcs_confirmed, c.no_dlcs_confirmed, c.source, c.created_at_ms FROM campaign_configurations c JOIN chats h ON h.configuration_id=c.id AND h.campaign_id=c.campaign_id WHERE h.campaign_id=?1 AND h.id=?2", params![campaign_id, chat_id], |row| Ok((Configuration { id: row.get(0)?, campaign_id: row.get(1)?, game_version: row.get(2)?, active_dlcs: vec![], dlcs_confirmed: row.get(4)?, no_dlcs_confirmed: row.get(5)?, source: row.get(6)?, created_at_ms: row.get(7)? }, row.get::<_, String>(3)?))).optional().map_err(failure)?.ok_or("Selecione um chat pertencente à campanha.")?;
        let (mut config, dlcs) = raw;
        config.active_dlcs = serde_json::from_str(&dlcs).map_err(failure)?;
        let name = self
            .connection
            .query_row(
                "SELECT name FROM campaigns WHERE id=?1",
                [campaign_id],
                |row| row.get(0),
            )
            .map_err(failure)?;
        NewCampaign {
            name,
            game_version: config.game_version.clone(),
            active_dlcs: config.active_dlcs.clone(),
            dlcs_confirmed: config.dlcs_confirmed,
            no_dlcs_confirmed: config.no_dlcs_confirmed,
        }
        .validate()?;
        Ok(config)
    }
    pub fn create_campaign(&mut self, input: NewCampaign) -> Result<Workspace, String> {
        let input = input.validate()?;
        let campaign = entity_id("campaign");
        let config = entity_id("configuration");
        let chat = entity_id("chat");
        let timestamp = now();
        let tx = self.connection.transaction().map_err(failure)?;
        tx.execute(
            "INSERT INTO campaigns VALUES (?1,?2,?3)",
            params![campaign, input.name, timestamp],
        )
        .map_err(failure)?;
        tx.execute(
            "INSERT INTO campaign_configurations VALUES (?1,?2,?3,?4,1,?5,'player',?6)",
            params![
                config,
                campaign,
                input.game_version,
                serde_json::to_string(&input.active_dlcs).map_err(failure)?,
                input.no_dlcs_confirmed,
                timestamp
            ],
        )
        .map_err(failure)?;
        tx.execute("INSERT INTO chats (id,campaign_id,configuration_id,title,created_at_ms) VALUES (?1,?2,?3,'Primeira conversa',?4)", params![chat, campaign, config, timestamp]).map_err(failure)?;
        tx.execute(
            "INSERT OR REPLACE INTO local_selection VALUES (1,?1,?2)",
            params![campaign, chat],
        )
        .map_err(failure)?;
        tx.commit().map_err(failure)?;
        self.workspace()
    }
    pub fn select_chat(&self, campaign_id: &str, chat_id: &str) -> Result<Workspace, String> {
        self.analysis_context(campaign_id, chat_id)?;
        self.connection
            .execute(
                "INSERT OR REPLACE INTO local_selection VALUES (1,?1,?2)",
                params![campaign_id, chat_id],
            )
            .map_err(failure)?;
        self.workspace()
    }
    pub fn select_campaign(&self, campaign_id: &str) -> Result<Workspace, String> {
        let chat: String = self
            .connection
            .query_row(
                "SELECT id FROM chats WHERE campaign_id=?1 ORDER BY created_at_ms, id LIMIT 1",
                [campaign_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(failure)?
            .ok_or("Campanha indisponível.")?;
        self.select_chat(campaign_id, &chat)
    }
    pub fn create_chat(&mut self, campaign_id: &str, title: &str) -> Result<Workspace, String> {
        let title = bounded_text(title, 120, "Informe o título do chat (até 120 bytes).")?;
        let config: String = self.connection.query_row("SELECT id FROM campaign_configurations WHERE campaign_id=?1 ORDER BY created_at_ms DESC, id DESC LIMIT 1", [campaign_id], |row| row.get(0)).optional().map_err(failure)?.ok_or("Campanha indisponível.")?;
        let chat = entity_id("chat");
        let tx = self.connection.transaction().map_err(failure)?;
        tx.execute("INSERT INTO chats (id,campaign_id,configuration_id,title,created_at_ms) VALUES (?1,?2,?3,?4,?5)", params![chat, campaign_id, config, title, now()]).map_err(failure)?;
        tx.execute(
            "INSERT OR REPLACE INTO local_selection VALUES (1,?1,?2)",
            params![campaign_id, chat],
        )
        .map_err(failure)?;
        tx.commit().map_err(failure)?;
        self.workspace()
    }
    pub fn save_draft(
        &self,
        campaign_id: &str,
        chat_id: &str,
        revision: i64,
        draft: &str,
    ) -> Result<Workspace, String> {
        if draft.len() > 16_000 || draft.contains('\0') {
            return Err(
                "O rascunho deve ter até 16.000 bytes e não pode conter caracteres nulos.".into(),
            );
        }
        let changed = self.connection.execute("UPDATE chats SET draft=?1, revision=revision+1 WHERE campaign_id=?2 AND id=?3 AND revision=?4", params![draft, campaign_id, chat_id, revision]).map_err(failure)?;
        if changed != 1 {
            return Err(
                "Chat indisponível ou alterado. Reabra o chat antes de salvar novamente.".into(),
            );
        }
        self.workspace()
    }
}
