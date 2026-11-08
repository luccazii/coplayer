use super::campaigns::{entity_id, now};
use super::*;
use crate::campaign::{DraftTarget, LocalMessage, SubmitMessage, Workspace};

impl Store {
    pub fn selected_draft(&self) -> Result<DraftTarget, String> {
        self.connection.query_row(
            "SELECT s.campaign_id, s.chat_id, c.draft_generation FROM local_selection s JOIN chats c ON c.id=s.chat_id AND c.campaign_id=s.campaign_id WHERE singleton=1",
            [], |row| Ok(DraftTarget { campaign_id: row.get(0)?, chat_id: row.get(1)?, generation: row.get(2)? })
        ).optional().map_err(failure)?.ok_or("Selecione uma campanha e um chat antes de capturar.".into())
    }
    pub fn validate_draft(&self, target: &DraftTarget) -> Result<(), String> {
        self.analysis_context(&target.campaign_id, &target.chat_id)?;
        let generation: i64 = self
            .connection
            .query_row(
                "SELECT draft_generation FROM chats WHERE id=?1",
                [&target.chat_id],
                |r| r.get(0),
            )
            .map_err(failure)?;
        if generation != target.generation {
            return Err(
                "O rascunho já foi enviado. Capture novamente para a nova mensagem.".into(),
            );
        }
        Ok(())
    }
    pub fn capture_destination(&self, target: &DraftTarget) -> Result<(), String> {
        self.validate_draft(target)?;
        let rows = self.scoped_rows(None, Some(&target.chat_id), None)?;
        if rows.len() >= MAX_ATTACHMENTS
            || rows.iter().map(|(item, _)| item.bytes).sum::<u64>() >= MAX_TOTAL_BYTES
        {
            return Err(
                "O rascunho atingiu o limite de anexos. Remova um antes de capturar.".into(),
            );
        }
        Ok(())
    }
    pub fn add_chat_capture(
        &mut self,
        target: &DraftTarget,
        pending: PendingAttachment,
    ) -> Result<Attachment, String> {
        self.validate_draft(target)?;
        self.add_scoped(Source::Capture, Some(&target.chat_id), vec![pending])?
            .pop()
            .ok_or("Nenhuma captura foi armazenada.".into())
    }
    pub fn remove_chat_attachment(
        &mut self,
        target: &DraftTarget,
        id: &str,
    ) -> Result<Workspace, String> {
        self.validate_draft(target)?;
        self.remove_scoped(None, Some(&target.chat_id), id)?;
        self.workspace()
    }
    pub(super) fn messages(&self, chat_id: &str) -> Result<Vec<LocalMessage>, String> {
        let mut statement = self.connection.prepare("SELECT id, campaign_id, chat_id, configuration_id, text, status, created_at_ms, (SELECT count(*) FROM attachments WHERE message_id=messages.id) FROM messages WHERE chat_id=?1 ORDER BY created_at_ms, rowid").map_err(failure)?;
        let messages = statement
            .query_map([chat_id], |row| {
                Ok(LocalMessage {
                    id: row.get(0)?,
                    campaign_id: row.get(1)?,
                    chat_id: row.get(2)?,
                    configuration_id: row.get(3)?,
                    text: row.get(4)?,
                    status: row.get(5)?,
                    created_at_ms: row.get(6)?,
                    attachment_count: row.get(7)?,
                })
            })
            .map_err(failure)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(failure)?;
        Ok(messages)
    }
    pub fn message_attachments(
        &self,
        campaign_id: &str,
        chat_id: &str,
        message_id: &str,
    ) -> Result<Vec<Attachment>, String> {
        self.analysis_context(campaign_id, chat_id)?;
        let belongs: bool = self.connection.query_row("SELECT EXISTS(SELECT 1 FROM messages WHERE campaign_id=?1 AND chat_id=?2 AND id=?3)", params![campaign_id,chat_id,message_id], |r| r.get(0)).map_err(failure)?;
        if !belongs {
            return Err("Mensagem indisponível neste chat.".into());
        }
        self.previews(self.scoped_rows(None, Some(chat_id), Some(message_id))?)
    }
    pub fn submit_message(&mut self, input: SubmitMessage) -> Result<Workspace, String> {
        let target = &input.target;
        let text = input.text.trim();
        if input.text.len() > 16_000 || input.text.contains('\0') {
            return Err(
                "A mensagem deve ter até 16.000 bytes e não pode conter caracteres nulos.".into(),
            );
        }
        if input.attachment_ids.len() > MAX_ATTACHMENTS
            || input.attachment_ids.iter().collect::<HashSet<_>>().len()
                != input.attachment_ids.len()
        {
            return Err("Lista de anexos inválida.".into());
        }
        let configuration = self.analysis_context(&target.campaign_id, &target.chat_id)?;
        // Retry of the same explicit submission must not duplicate the message.
        let existing: Option<(String, String)> = self
            .connection
            .query_row(
                "SELECT id,text FROM messages WHERE chat_id=?1 AND draft_generation=?2",
                params![target.chat_id, target.generation],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(failure)?;
        if let Some((id, stored_text)) = existing {
            let rows = self.scoped_rows(None, Some(&target.chat_id), Some(&id))?;
            if stored_text == text && same_ids(&rows, &input.attachment_ids) {
                return self.workspace();
            }
            return Err("Esta versão do rascunho já foi registrada com outro conteúdo.".into());
        }
        self.validate_draft(target)?;
        let rows = self.scoped_rows(None, Some(&target.chat_id), None)?;
        if !same_ids(&rows, &input.attachment_ids) {
            return Err(
                "Os anexos mudaram. Revise as capturas antes de registrar a mensagem.".into(),
            );
        }
        if text.is_empty() && rows.is_empty() {
            return Err("Escreva uma mensagem ou adicione uma captura.".into());
        }
        for (attachment, hash) in &rows {
            self.verified_bytes(attachment, hash)?;
        }
        let id = entity_id("message");
        let tx = self.connection.transaction().map_err(failure)?;
        let changed = tx.execute("UPDATE chats SET draft='', revision=revision+1, draft_generation=draft_generation+1 WHERE id=?1 AND campaign_id=?2 AND revision=?3 AND draft_generation=?4", params![target.chat_id,target.campaign_id,input.revision,target.generation]).map_err(failure)?;
        if changed != 1 {
            return Err("O rascunho foi alterado. Reabra o chat e revise a mensagem.".into());
        }
        tx.execute(
            "INSERT INTO messages VALUES (?1,?2,?3,?4,?5,?6,'local_only',?7)",
            params![
                id,
                target.campaign_id,
                target.chat_id,
                configuration.id,
                target.generation,
                text,
                now()
            ],
        )
        .map_err(failure)?;
        for attachment in &input.attachment_ids {
            tx.execute("UPDATE attachments SET message_id=?1 WHERE id=?2 AND owner_chat_id=?3 AND message_id IS NULL AND deleted=0", params![id,attachment,target.chat_id]).map_err(failure)?;
        }
        tx.commit().map_err(failure)?;
        self.workspace()
    }
}
fn same_ids(rows: &[(Attachment, String)], ids: &[String]) -> bool {
    rows.len() == ids.len() && rows.iter().all(|(item, _)| ids.contains(&item.id))
}
