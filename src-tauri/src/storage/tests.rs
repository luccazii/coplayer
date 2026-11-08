use super::*;

struct TestDirectory(PathBuf);
impl TestDirectory {
    fn new() -> Self {
        Self(std::env::temp_dir().join(format!("coplayer-test-{}", identifier())))
    }
    fn open(&self) -> Store {
        Store::open(&self.0).unwrap()
    }
}
impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn png() -> Vec<u8> {
    let image = image::RgbImage::from_pixel(12, 8, image::Rgb([21, 87, 163]));
    let mut data = Cursor::new(Vec::new());
    image.write_to(&mut data, ImageFormat::Png).unwrap();
    data.into_inner()
}
fn pending(name: &str) -> PendingAttachment {
    PendingAttachment {
        name: name.into(),
        data: png(),
    }
}

#[test]
fn reopen_preserves_identity_metadata_and_bytes_without_external_paths() {
    let directory = TestDirectory::new();
    let id = {
        let mut store = directory.open();
        let item = store
            .add(Source::Import, vec![pending("test.png")])
            .unwrap()
            .remove(0);
        assert_eq!((item.width, item.height), (12, 8));
        assert!(item.available);
        let json = serde_json::to_value(&item).unwrap();
        assert!(json.get("path").is_none());
        item.id
    };
    let store = directory.open();
    let restored = store.list(Source::Import).unwrap();
    assert_eq!(restored.len(), 1);
    assert_eq!(restored[0].id, id);
    assert!(restored[0].available);
    assert_eq!(
        fs::read(store.resolve(Source::Import, &[id]).unwrap().remove(0)).unwrap(),
        png()
    );
}

#[test]
fn invalid_batch_never_commits_partial_attachments() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    for data in [
        b"<svg></svg>".to_vec(),
        b"\x89PNG\r\n\x1a\n".to_vec(),
        vec![0; MAX_IMAGE_BYTES as usize + 1],
    ] {
        assert!(
            store
                .add(
                    Source::Import,
                    vec![
                        pending("valid.png"),
                        PendingAttachment {
                            name: "bad.png".into(),
                            data
                        }
                    ]
                )
                .is_err()
        );
        assert!(store.list(Source::Import).unwrap().is_empty());
        assert_eq!(fs::read_dir(&store.directory).unwrap().count(), 0);
    }
    let large = image::RgbImage::new(MAX_DIMENSION + 1, 1);
    let mut bytes = Cursor::new(Vec::new());
    large.write_to(&mut bytes, ImageFormat::Png).unwrap();
    assert!(validate(&bytes.into_inner()).is_err());
}

#[test]
fn count_scope_and_identifier_checks_reject_unowned_files() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let imported = store
        .add(
            Source::Import,
            (0..MAX_ATTACHMENTS).map(|_| pending("a.png")).collect(),
        )
        .unwrap();
    assert!(
        store
            .add(Source::Import, vec![pending("extra.png")])
            .is_err()
    );
    let capture = store
        .add(Source::Capture, vec![pending("game")])
        .unwrap()
        .remove(0);
    assert!(
        store
            .resolve(Source::Import, std::slice::from_ref(&capture.id))
            .is_err()
    );
    assert!(store.remove(Source::Import, &capture.id).is_err());
    assert!(
        store
            .resolve(
                Source::Import,
                &[imported[0].id.clone(), imported[0].id.clone()]
            )
            .is_err()
    );
    let outside = directory.0.join("keep.png");
    fs::write(&outside, png()).unwrap();
    assert!(store.remove(Source::Import, "../keep").is_err());
    assert!(outside.exists());
}

#[test]
fn missing_and_tampered_images_are_visible_but_not_sendable() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let items = store
        .add(Source::Import, vec![pending("missing"), pending("changed")])
        .unwrap();
    fs::remove_file(store.path(&items[0].id, &items[0].mime).unwrap()).unwrap();
    fs::write(
        store.path(&items[1].id, &items[1].mime).unwrap(),
        b"changed",
    )
    .unwrap();
    assert!(
        store
            .list(Source::Import)
            .unwrap()
            .iter()
            .all(|item| !item.available && item.preview_data_url.is_empty())
    );
    for item in items {
        assert!(
            store
                .resolve(Source::Import, std::slice::from_ref(&item.id))
                .is_err()
        );
        store.remove(Source::Import, &item.id).unwrap();
    }
    assert!(store.list(Source::Import).unwrap().is_empty());
}

#[test]
fn recovery_finishes_deletion_and_removes_only_owned_orphans() {
    let directory = TestDirectory::new();
    let (deleted_path, orphan, unrelated) = {
        let mut store = directory.open();
        let item = store
            .add(Source::Capture, vec![pending("removed")])
            .unwrap()
            .remove(0);
        let deleted_path = store.path(&item.id, &item.mime).unwrap();
        store
            .connection
            .execute("UPDATE attachments SET deleted=1 WHERE id=?1", [&item.id])
            .unwrap();
        let orphan = store.directory.join(format!("{}.png", identifier()));
        let unrelated = store.directory.join("keep.png");
        fs::write(&orphan, png()).unwrap();
        fs::write(&unrelated, png()).unwrap();
        (deleted_path, orphan, unrelated)
    };
    let store = directory.open();
    assert!(store.list(Source::Capture).unwrap().is_empty());
    assert!(!deleted_path.exists());
    assert!(!orphan.exists());
    assert!(unrelated.exists());
}

#[test]
fn failed_transaction_cleans_files_and_preserves_existing_records() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let existing = store
        .add(Source::Import, vec![pending("existing")])
        .unwrap()
        .remove(0);
    store.connection.execute_batch("CREATE TRIGGER reject_bad BEFORE INSERT ON attachments WHEN NEW.name='bad' BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
    assert!(
        store
            .add(Source::Import, vec![pending("first"), pending("bad")])
            .is_err()
    );
    let items = store.list(Source::Import).unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, existing.id);
    assert_eq!(fs::read_dir(&store.directory).unwrap().count(), 1);
}

#[test]
fn future_schema_is_not_downgraded() {
    let directory = TestDirectory::new();
    {
        let store = directory.open();
        store
            .connection
            .pragma_update(None, "user_version", 99)
            .unwrap();
    }
    assert!(Store::open(&directory.0).is_err());
}

#[cfg(unix)]
#[test]
fn private_files_and_symlink_rejection() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let item = store
        .add(Source::Import, vec![pending("private")])
        .unwrap()
        .remove(0);
    let path = store.path(&item.id, &item.mime).unwrap();
    assert_eq!(
        fs::metadata(path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(
        fs::metadata(directory.0.join("coplayer.sqlite3"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    let link = directory.0.join("link.png");
    let original = directory.0.join("original.png");
    fs::write(&original, png()).unwrap();
    symlink(&original, &link).unwrap();
    assert!(read_image(&link).is_err());
}

#[test]
fn aggregate_byte_limit_is_enforced_before_any_write() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let mut data = png();
    data.resize(MAX_IMAGE_BYTES as usize, 0);
    let batch = (0..3)
        .map(|_| PendingAttachment {
            name: "large.png".into(),
            data: data.clone(),
        })
        .collect();
    assert_eq!(store.add(Source::Import, batch).unwrap().len(), 3);
    assert!(
        store
            .add(Source::Import, vec![pending("over-total")])
            .is_err()
    );
    assert_eq!(store.list(Source::Import).unwrap().len(), 3);
    assert_eq!(fs::read_dir(&store.directory).unwrap().count(), 3);
}

fn campaign_input(name: &str) -> crate::campaign::NewCampaign {
    crate::campaign::NewCampaign {
        name: name.into(),
        game_version: "1.15.0".into(),
        active_dlcs: vec![],
        dlcs_confirmed: true,
        no_dlcs_confirmed: true,
    }
}

#[test]
fn campaign_requires_name_version_and_explicit_dlc_confirmation() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let mut invalid = Vec::new();
    let mut input = campaign_input(" ");
    invalid.push(input);
    input = campaign_input("A");
    input.game_version = " ".into();
    invalid.push(input);
    input = campaign_input("A");
    input.dlcs_confirmed = false;
    invalid.push(input);
    input = campaign_input("A");
    input.no_dlcs_confirmed = false;
    invalid.push(input);
    input = campaign_input("A");
    input.active_dlcs = vec!["DLC".into()];
    invalid.push(input);
    input = campaign_input("A");
    input.active_dlcs = vec!["DLC".into(), " dlc ".into()];
    input.no_dlcs_confirmed = false;
    invalid.push(input);
    input = campaign_input("A");
    input.active_dlcs = vec![" ".into()];
    input.no_dlcs_confirmed = false;
    invalid.push(input);
    input = campaign_input(&"é".repeat(61));
    invalid.push(input);
    for input in invalid {
        assert!(store.create_campaign(input).is_err());
        let state = store.workspace().unwrap();
        assert!(state.campaigns.is_empty());
        assert!(state.chat_id.is_none());
    }
    let state = store
        .create_campaign(campaign_input("  Sem DLCs  "))
        .unwrap();
    assert_eq!(state.campaigns[0].name, "Sem DLCs");
    assert!(state.configuration.unwrap().no_dlcs_confirmed);
    let mut input = campaign_input("Com DLCs");
    input.active_dlcs = vec!["  DLC de teste  ".into()];
    input.no_dlcs_confirmed = false;
    let state = store.create_campaign(input).unwrap();
    assert_eq!(state.configuration.unwrap().active_dlcs, ["DLC de teste"]);
}

#[test]
fn campaign_chats_and_drafts_reopen_without_crossing_ownership() {
    let directory = TestDirectory::new();
    let (a, a_chat, b, b_chat, config) = {
        let mut store = directory.open();
        let state = store.create_campaign(campaign_input("A")).unwrap();
        let a = state.campaign_id.unwrap();
        let a_chat = state.chat_id.unwrap();
        let config = state.configuration.unwrap().id;
        store
            .save_draft(&a, &a_chat, 0, "Rascunho A\ncom contexto")
            .unwrap();
        let state = store.create_campaign(campaign_input("B")).unwrap();
        let b = state.campaign_id.unwrap();
        let b_chat = state.chat_id.unwrap();
        store.save_draft(&b, &b_chat, 0, "Rascunho B").unwrap();
        assert!(store.analysis_context(&a, &b_chat).is_err());
        assert!(store.select_chat(&b, &a_chat).is_err());
        assert!(store.save_draft(&b, &a_chat, 1, "misturado").is_err());
        assert!(store.save_draft(&a, &a_chat, 0, "antigo").is_err());
        assert!(
            store
                .save_draft(&a, &a_chat, 1, &"x".repeat(16_001))
                .is_err()
        );
        assert!(store.select_campaign("missing").is_err());
        assert!(store.create_chat("missing", "Chat").is_err());
        assert!(store.create_chat(&a, " ").is_err());
        let state = store.create_chat(&a, "Outra conversa").unwrap();
        assert_eq!(state.chats.len(), 2);
        assert!(
            state
                .chats
                .iter()
                .find(|c| Some(&c.id) == state.chat_id.as_ref())
                .unwrap()
                .draft
                .is_empty()
        );
        assert_eq!(state.configuration.unwrap().id, config);
        store.select_chat(&b, &b_chat).unwrap();
        (a, a_chat, b, b_chat, config)
    };
    let store = directory.open();
    let state = store.workspace().unwrap();
    assert_eq!(state.campaign_id.as_ref(), Some(&b));
    assert_eq!(state.chat_id.as_ref(), Some(&b_chat));
    assert_eq!(state.chats.len(), 1);
    assert_eq!(state.chats[0].draft, "Rascunho B");
    let state = store.select_chat(&a, &a_chat).unwrap();
    assert_eq!(state.configuration.unwrap().id, config);
    assert_eq!(state.chats[0].draft, "Rascunho A\ncom contexto");
    assert_eq!(state.chats[0].revision, 1);
    assert!(
        store
            .connection
            .execute(
                "UPDATE campaign_configurations SET game_version='changed' WHERE id=?1",
                [&config]
            )
            .is_err()
    );
    assert!(
        store
            .connection
            .execute(
                "UPDATE chats SET configuration_id=?1 WHERE id=?2",
                params![config, b_chat]
            )
            .is_err()
    );
}

#[test]
fn campaign_creation_rolls_back_all_records_and_selection() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    let first = store.create_campaign(campaign_input("Existing")).unwrap();
    store.connection.execute_batch("CREATE TRIGGER reject_chat BEFORE INSERT ON chats BEGIN SELECT RAISE(ABORT, 'test failure'); END;").unwrap();
    assert!(store.create_campaign(campaign_input("Rejected")).is_err());
    assert!(
        store
            .create_chat(first.campaign_id.as_ref().unwrap(), "Rejected chat")
            .is_err()
    );
    let state = store.workspace().unwrap();
    assert_eq!(state.campaigns.len(), 1);
    assert_eq!(state.chat_id, first.chat_id);
    assert_eq!(state.chats.len(), 1);
    let count: i64 = store
        .connection
        .query_row("SELECT count(*) FROM campaign_configurations", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn v1_migration_preserves_attachment_files_and_is_atomic() {
    let directory = TestDirectory::new();
    private_directory(&directory.0).unwrap();
    let database = directory.0.join("coplayer.sqlite3");
    {
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(include_str!("schema.sql"))
            .unwrap();
        let attachments = directory.0.join("attachments");
        private_directory(&attachments).unwrap();
        let id = identifier();
        let bytes = png();
        fs::write(attachments.join(format!("{id}.png")), &bytes).unwrap();
        connection.execute("INSERT INTO attachments VALUES (?1,'import','legacy.png','image/png',?2,12,8,?3,1,0)", params![id,bytes.len() as i64,digest(&bytes)]).unwrap();
        // Inject a collision midway through v2 to prove all earlier DDL rolls back.
        connection
            .execute_batch("CREATE TABLE chats (legacy TEXT);")
            .unwrap();
    }
    assert!(Store::open(&directory.0).is_err());
    {
        let connection = Connection::open(&database).unwrap();
        let version: u32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(version, 1);
        let count: i64 = connection
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE name='campaigns'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
        connection.execute_batch("DROP TABLE chats;").unwrap();
    }
    let store = directory.open();
    let images = store.list(Source::Import).unwrap();
    assert_eq!(images.len(), 1);
    assert!(images[0].available);
    assert_eq!(images[0].name, "legacy.png");
    assert!(store.workspace().unwrap().campaigns.is_empty());
    assert_eq!(
        store
            .connection
            .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
            .unwrap(),
        3
    );
}

fn submission(
    target: &crate::campaign::DraftTarget,
    revision: i64,
    text: &str,
    ids: &[String],
) -> crate::campaign::SubmitMessage {
    crate::campaign::SubmitMessage {
        target: target.clone(),
        revision,
        text: text.into(),
        attachment_ids: ids.to_vec(),
    }
}

#[test]
fn chat_capture_ownership_survives_selection_changes_and_is_separate_from_proofs() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    assert!(store.selected_draft().is_err());
    store.create_campaign(campaign_input("A")).unwrap();
    let a = store.selected_draft().unwrap();
    store.create_campaign(campaign_input("B")).unwrap();
    let b = store.selected_draft().unwrap();
    // The window operation started for A and completed after B was selected.
    let image = store.add_chat_capture(&a, pending("A.png")).unwrap();
    assert!(store.workspace().unwrap().attachments.is_empty());
    assert!(store.list(Source::Capture).unwrap().is_empty());
    assert!(
        store
            .resolve(Source::Capture, std::slice::from_ref(&image.id))
            .is_err()
    );
    assert!(store.remove(Source::Capture, &image.id).is_err());
    assert!(store.remove_chat_attachment(&b, &image.id).is_err());
    let wrong = crate::campaign::DraftTarget {
        campaign_id: b.campaign_id.clone(),
        ..a.clone()
    };
    assert!(
        store
            .add_chat_capture(&wrong, pending("wrong.png"))
            .is_err()
    );
    assert_eq!(
        store
            .select_chat(&a.campaign_id, &a.chat_id)
            .unwrap()
            .attachments[0]
            .id,
        image.id
    );
    let proof = store
        .add(Source::Capture, vec![pending("proof.png")])
        .unwrap()
        .remove(0);
    assert!(store.remove_chat_attachment(&a, &proof.id).is_err());
    store.remove_chat_attachment(&a, &image.id).unwrap();
    assert!(!store.path(&image.id, &image.mime).unwrap().exists());
    assert!(store.workspace().unwrap().attachments.is_empty());
    assert_eq!(store.list(Source::Capture).unwrap().len(), 1);
}

#[test]
fn explicit_local_submission_is_atomic_idempotent_and_rejects_late_capture() {
    let directory = TestDirectory::new();
    let (target, message_id, image_id) = {
        let mut store = directory.open();
        store.create_campaign(campaign_input("A")).unwrap();
        let target = store.selected_draft().unwrap();
        assert!(
            store
                .submit_message(submission(&target, 0, " ", &[]))
                .is_err()
        );
        let first = store
            .add_chat_capture(&target, pending("first.png"))
            .unwrap();
        let removed = store
            .add_chat_capture(&target, pending("removed.png"))
            .unwrap();
        store.remove_chat_attachment(&target, &removed.id).unwrap();
        let ids = vec![first.id.clone()];
        assert!(
            store
                .submit_message(submission(&target, 0, "question", &[]))
                .is_err()
        );
        assert!(
            store
                .submit_message(submission(
                    &target,
                    0,
                    "question",
                    &[first.id.clone(), first.id.clone()]
                ))
                .is_err()
        );
        assert!(
            store
                .submit_message(submission(
                    &target,
                    0,
                    "question",
                    std::slice::from_ref(&removed.id)
                ))
                .is_err()
        );
        assert!(
            store
                .submit_message(submission(&target, 1, "question", &ids))
                .is_err()
        );
        let state = store
            .submit_message(submission(&target, 0, "  question\ncontext  ", &ids))
            .unwrap();
        assert!(state.attachments.is_empty());
        assert!(state.chats[0].draft.is_empty());
        assert_eq!(state.chats[0].draft_generation, 1);
        assert_eq!(state.messages.len(), 1);
        assert_eq!(state.messages[0].status, "local_only");
        assert_eq!(state.messages[0].text, "question\ncontext");
        assert_eq!(state.messages[0].attachment_count, 1);
        let id = state.messages[0].id.clone();
        assert_eq!(
            store
                .submit_message(submission(&target, 0, "question\ncontext", &ids))
                .unwrap()
                .messages
                .len(),
            1
        );
        assert!(
            store
                .submit_message(submission(&target, 0, "different", &ids))
                .is_err()
        );
        assert!(
            store
                .add_chat_capture(&target, pending("late.png"))
                .is_err()
        );
        assert!(store.remove_chat_attachment(&target, &first.id).is_err());
        let next = store.selected_draft().unwrap();
        assert!(store.remove_chat_attachment(&next, &first.id).is_err());
        let images = store
            .message_attachments(&target.campaign_id, &target.chat_id, &id)
            .unwrap();
        assert_eq!(images[0].id, first.id);
        assert!(images[0].available);
        (target, id, first.id)
    };
    let mut store = directory.open();
    let restored = store.workspace().unwrap();
    assert_eq!(restored.messages[0].id, message_id);
    assert_eq!(restored.messages[0].status, "local_only");
    assert!(restored.attachments.is_empty());
    assert_eq!(
        store
            .message_attachments(&target.campaign_id, &target.chat_id, &message_id)
            .unwrap()[0]
            .id,
        image_id
    );
    store.create_campaign(campaign_input("B")).unwrap();
    let b = store.selected_draft().unwrap();
    assert!(
        store
            .message_attachments(&b.campaign_id, &b.chat_id, &message_id)
            .is_err()
    );
    assert!(store.workspace().unwrap().messages.is_empty());
}

#[test]
fn new_capture_requires_review_and_tampered_file_cannot_be_submitted() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    store.create_campaign(campaign_input("A")).unwrap();
    let target = store.selected_draft().unwrap();
    let first = store
        .add_chat_capture(&target, pending("first.png"))
        .unwrap();
    let second = store
        .add_chat_capture(&target, pending("second.png"))
        .unwrap();
    assert!(
        store
            .submit_message(submission(
                &target,
                0,
                "question",
                std::slice::from_ref(&first.id)
            ))
            .is_err()
    );
    let path = store.path(&second.id, &second.mime).unwrap();
    fs::write(&path, b"tampered").unwrap();
    let ids = [first.id.clone(), second.id.clone()];
    assert!(
        store
            .submit_message(submission(&target, 0, "question", &ids))
            .is_err()
    );
    assert!(store.workspace().unwrap().messages.is_empty());
    assert_eq!(store.selected_draft().unwrap().generation, 0);
    assert!(!store.workspace().unwrap().attachments[1].available);
    fs::remove_file(path).unwrap();
    assert!(
        store
            .submit_message(submission(&target, 0, "question", &ids))
            .is_err()
    );
    store.remove_chat_attachment(&target, &second.id).unwrap();
    store
        .submit_message(submission(&target, 0, "", std::slice::from_ref(&first.id)))
        .unwrap();
}

#[test]
fn capture_limits_are_per_draft_and_submission_releases_capacity() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    store.create_campaign(campaign_input("A")).unwrap();
    let a = store.selected_draft().unwrap();
    let mut ids = Vec::new();
    for _ in 0..MAX_ATTACHMENTS {
        ids.push(
            store
                .add_chat_capture(&a, pending("capture.png"))
                .unwrap()
                .id,
        );
    }
    assert!(store.capture_destination(&a).is_err());
    assert!(store.add_chat_capture(&a, pending("extra.png")).is_err());
    store.create_campaign(campaign_input("B")).unwrap();
    let b = store.selected_draft().unwrap();
    assert!(store.capture_destination(&b).is_ok());
    store
        .add_chat_capture(&b, pending("independent.png"))
        .unwrap();
    store.select_chat(&a.campaign_id, &a.chat_id).unwrap();
    store
        .submit_message(submission(&a, 0, "four images", &ids))
        .unwrap();
    let next = store.selected_draft().unwrap();
    assert!(store.capture_destination(&next).is_ok());
    store
        .add_chat_capture(&next, pending("new draft.png"))
        .unwrap();
    assert_eq!(store.workspace().unwrap().attachments.len(), 1);
    assert_eq!(store.workspace().unwrap().messages[0].attachment_count, 4);
}

#[test]
fn failed_submission_preserves_draft_and_attachment_ownership() {
    let directory = TestDirectory::new();
    let mut store = directory.open();
    store.create_campaign(campaign_input("A")).unwrap();
    let target = store.selected_draft().unwrap();
    store
        .save_draft(&target.campaign_id, &target.chat_id, 0, "saved text")
        .unwrap();
    let item = store
        .add_chat_capture(&target, pending("capture.png"))
        .unwrap();
    store.connection.execute_batch("CREATE TRIGGER fail_message BEFORE INSERT ON messages BEGIN SELECT RAISE(ABORT,'test failure'); END;").unwrap();
    assert!(
        store
            .submit_message(submission(
                &target,
                1,
                "saved text",
                std::slice::from_ref(&item.id)
            ))
            .is_err()
    );
    let state = store.workspace().unwrap();
    assert_eq!(state.chats[0].draft, "saved text");
    assert_eq!(state.chats[0].revision, 1);
    assert_eq!(state.chats[0].draft_generation, 0);
    assert_eq!(state.attachments[0].id, item.id);
    assert!(state.messages.is_empty());
}

#[test]
fn v3_migration_failure_preserves_v2_campaigns_and_recovers_on_retry() {
    let directory = TestDirectory::new();
    private_directory(&directory.0).unwrap();
    let database = directory.0.join("coplayer.sqlite3");
    {
        let connection = Connection::open(&database).unwrap();
        connection
            .execute_batch(include_str!("schema.sql"))
            .unwrap();
        connection
            .execute_batch(include_str!("campaigns.sql"))
            .unwrap();
        connection.execute_batch("INSERT INTO campaigns VALUES ('c','Preserved',1); INSERT INTO campaign_configurations VALUES ('v','c','1.15.0','[]',1,1,'player',1); INSERT INTO chats VALUES ('h','c','v','Chat','Saved',0,1); INSERT INTO local_selection VALUES (1,'c','h'); CREATE TABLE messages (collision TEXT);").unwrap();
    }
    assert!(Store::open(&directory.0).is_err());
    {
        let connection = Connection::open(&database).unwrap();
        assert_eq!(
            connection
                .pragma_query_value::<u32, _>(None, "user_version", |row| row.get(0))
                .unwrap(),
            2
        );
        assert!(
            connection
                .prepare("SELECT draft_generation FROM chats")
                .is_err()
        );
        connection.execute_batch("DROP TABLE messages;").unwrap();
    }
    let store = directory.open();
    let state = store.workspace().unwrap();
    assert_eq!(state.campaigns[0].name, "Preserved");
    assert_eq!(state.chats[0].draft, "Saved");
    assert_eq!(state.chats[0].draft_generation, 0);
    assert!(state.attachments.is_empty());
    assert!(state.messages.is_empty());
}
