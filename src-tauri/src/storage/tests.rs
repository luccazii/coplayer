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
