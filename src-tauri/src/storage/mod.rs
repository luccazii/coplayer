//! Local attachment ownership. No platform or provider API belongs in this module.
mod campaigns;
mod messages;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use image::{ImageFormat, ImageReader, Limits};
use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const MAX_ATTACHMENTS: usize = 4;
pub const MAX_IMAGE_BYTES: u64 = 8 * 1024 * 1024;
pub const MAX_TOTAL_BYTES: u64 = 24 * 1024 * 1024;
pub const MAX_DIMENSION: u32 = 4096;

#[derive(Clone, Copy)]
pub enum Source {
    Capture,
    Import,
}
impl Source {
    fn value(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Import => "import",
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub mime: String,
    pub bytes: u64,
    pub width: u32,
    pub height: u32,
    pub created_at_ms: i64,
    pub available: bool,
    pub preview_data_url: String,
}

pub struct PendingAttachment {
    pub name: String,
    pub data: Vec<u8>,
}
pub struct StorageState(pub Mutex<Store>);
pub struct Store {
    connection: Connection,
    directory: PathBuf,
}

fn failure(error: impl std::fmt::Display) -> String {
    format!("Falha no armazenamento local: {error}")
}
pub fn private_directory(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(failure)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).map_err(failure)?;
    }
    Ok(())
}
fn private_file(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).map_err(failure)?;
    }
    Ok(())
}
pub fn identifier() -> String {
    static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!(
        "attachment-{time}-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    )
}
fn valid_id(id: &str) -> bool {
    id.starts_with("attachment-")
        && id.len() < 100
        && id[11..].bytes().all(|c| c.is_ascii_digit() || c == b'-')
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Bounded reads also reject symlinks and files changed during the read.
pub fn read_image(path: &Path) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|_| "Imagem indisponível.".to_string())?;
    if !metadata.file_type().is_file() || metadata.len() == 0 || metadata.len() > MAX_IMAGE_BYTES {
        return Err("Selecione um arquivo PNG ou JPEG de até 8 MiB.".into());
    }
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(failure)?
        .take(MAX_IMAGE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(failure)?;
    if bytes.len() as u64 != metadata.len() {
        return Err("A imagem mudou durante a leitura; selecione novamente.".into());
    }
    Ok(bytes)
}
fn validate(data: &[u8]) -> Result<(&'static str, u32, u32), String> {
    if data.is_empty() || data.len() as u64 > MAX_IMAGE_BYTES {
        return Err("Cada imagem deve ter até 8 MiB.".into());
    }
    let format =
        image::guess_format(data).map_err(|_| "Imagem PNG ou JPEG inválida.".to_string())?;
    let mime = match format {
        ImageFormat::Png => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        _ => return Err("Somente PNG e JPEG são aceitos.".into()),
    };
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_DIMENSION);
    limits.max_image_height = Some(MAX_DIMENSION);
    limits.max_alloc = Some(256 * 1024 * 1024);
    let mut reader = ImageReader::with_format(Cursor::new(data), format);
    reader.limits(limits);
    let image = reader
        .decode()
        .map_err(|_| "Imagem inválida ou acima de 4096 pixels por lado.".to_string())?;
    Ok((mime, image.width(), image.height()))
}

impl Store {
    pub fn open(root: &Path) -> Result<Self, String> {
        private_directory(root)?;
        let directory = root.join("attachments");
        private_directory(&directory)?;
        let database = root.join("coplayer.sqlite3");
        let mut connection = Connection::open(&database).map_err(failure)?;
        private_file(&database)?;
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(failure)?;
        connection.execute_batch("PRAGMA foreign_keys = ON; PRAGMA synchronous = FULL; PRAGMA locking_mode = EXCLUSIVE; BEGIN EXCLUSIVE; COMMIT;").map_err(failure)?;
        let version: u32 = connection
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(failure)?;
        if version > 3 {
            return Err("Este banco foi criado por uma versão mais recente do Coplayer.".into());
        }
        if version == 0 {
            let tx = connection.transaction().map_err(failure)?;
            tx.execute_batch(include_str!("schema.sql"))
                .map_err(failure)?;
            tx.commit().map_err(failure)?;
        }
        if version < 2 {
            let tx = connection.transaction().map_err(failure)?;
            tx.execute_batch(include_str!("campaigns.sql"))
                .map_err(failure)?;
            tx.commit().map_err(failure)?;
        }
        if version < 3 {
            let tx = connection.transaction().map_err(failure)?;
            tx.execute_batch(include_str!("messages.sql"))
                .map_err(failure)?;
            tx.commit().map_err(failure)?;
        }
        let store = Self {
            connection,
            directory,
        };
        store.recover_files()?;
        let staging = root.join("staging");
        private_directory(&staging)?;
        for entry in fs::read_dir(&staging).map_err(failure)? {
            let entry = entry.map_err(failure)?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if entry.file_type().map_err(failure)?.is_file()
                && name.strip_suffix(".png").is_some_and(valid_id)
            {
                fs::remove_file(entry.path()).map_err(failure)?;
            }
        }
        Ok(store)
    }
    fn path(&self, id: &str, mime: &str) -> Result<PathBuf, String> {
        if !valid_id(id) {
            return Err("Identificador de anexo inválido.".into());
        }
        let extension = match mime {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            _ => return Err("Formato de anexo inválido.".into()),
        };
        Ok(self.directory.join(format!("{id}.{extension}")))
    }
    // Deletion intent is committed before unlink. Crashes never resurrect an attachment.
    // Only this store's generated names are eligible for orphan cleanup.
    fn recover_files(&self) -> Result<(), String> {
        for entry in fs::read_dir(&self.directory).map_err(failure)? {
            let entry = entry.map_err(failure)?;
            if !entry.file_type().map_err(failure)?.is_file() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            let Some(id) = name
                .strip_suffix(".png")
                .or_else(|| name.strip_suffix(".jpg"))
            else {
                continue;
            };
            if !valid_id(id) {
                continue;
            }
            let live: bool = self
                .connection
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM attachments WHERE id = ?1 AND deleted = 0)",
                    [id],
                    |row| row.get(0),
                )
                .map_err(failure)?;
            if !live {
                fs::remove_file(entry.path()).map_err(failure)?;
            }
        }
        Ok(())
    }
    fn rows(&self, source: Source) -> Result<Vec<(Attachment, String)>, String> {
        self.scoped_rows(Some(source), None, None)
    }
    fn scoped_rows(
        &self,
        source: Option<Source>,
        chat_id: Option<&str>,
        message_id: Option<&str>,
    ) -> Result<Vec<(Attachment, String)>, String> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, mime, byte_count, width, height, created_at_ms, sha256 FROM attachments WHERE (?1 IS NULL OR source = ?1) AND owner_chat_id IS ?2 AND message_id IS ?3 AND deleted = 0 ORDER BY created_at_ms, rowid"
        ).map_err(failure)?;
        statement
            .query_map(
                params![source.map(Source::value), chat_id, message_id],
                |row| {
                    Ok((
                        Attachment {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            mime: row.get(2)?,
                            bytes: u64::from(row.get::<_, u32>(3)?),
                            width: row.get(4)?,
                            height: row.get(5)?,
                            created_at_ms: row.get(6)?,
                            available: false,
                            preview_data_url: String::new(),
                        },
                        row.get(7)?,
                    ))
                },
            )
            .map_err(failure)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(failure)
    }
    fn verified_bytes(&self, attachment: &Attachment, expected: &str) -> Result<Vec<u8>, String> {
        let bytes = read_image(&self.path(&attachment.id, &attachment.mime)?)?;
        if bytes.len() as u64 != attachment.bytes || digest(&bytes) != expected {
            return Err("O arquivo do anexo foi alterado; remova e importe novamente.".into());
        }
        Ok(bytes)
    }
    pub fn list(&self, source: Source) -> Result<Vec<Attachment>, String> {
        self.previews(self.rows(source)?)
    }
    fn previews(&self, rows: Vec<(Attachment, String)>) -> Result<Vec<Attachment>, String> {
        rows.into_iter()
            .map(|(mut attachment, hash)| {
                if let Ok(bytes) = self.verified_bytes(&attachment, &hash) {
                    attachment.available = true;
                    attachment.preview_data_url =
                        format!("data:{};base64,{}", attachment.mime, STANDARD.encode(bytes));
                }
                Ok(attachment)
            })
            .collect()
    }
    pub fn add(
        &mut self,
        source: Source,
        pending: Vec<PendingAttachment>,
    ) -> Result<Vec<Attachment>, String> {
        self.add_scoped(source, None, pending)
    }
    fn add_scoped(
        &mut self,
        source: Source,
        chat_id: Option<&str>,
        pending: Vec<PendingAttachment>,
    ) -> Result<Vec<Attachment>, String> {
        let existing = self.scoped_rows(
            if chat_id.is_some() {
                None
            } else {
                Some(source)
            },
            chat_id,
            None,
        )?;
        if existing.len() + pending.len() > MAX_ATTACHMENTS {
            return Err("São permitidos até 4 anexos; remova um antes de continuar.".into());
        }
        let mut total: u64 = existing.iter().map(|(item, _)| item.bytes).sum();
        let mut prepared = Vec::new();
        for item in pending {
            total = total.saturating_add(item.data.len() as u64);
            if total > MAX_TOTAL_BYTES {
                return Err("Os anexos devem somar no máximo 24 MiB.".into());
            }
            let (mime, width, height) = validate(&item.data)?;
            let attachment = Attachment {
                id: identifier(),
                name: item.name.chars().take(240).collect(),
                mime: mime.into(),
                bytes: item.data.len() as u64,
                width,
                height,
                created_at_ms: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(failure)?
                    .as_millis()
                    .try_into()
                    .map_err(failure)?,
                available: true,
                preview_data_url: format!("data:{mime};base64,{}", STANDARD.encode(&item.data)),
            };
            prepared.push((attachment, item.data));
        }
        let paths = prepared
            .iter()
            .map(|(item, _)| self.path(&item.id, &item.mime))
            .collect::<Result<Vec<_>, _>>()?;
        let mut created = Vec::new();
        let result = (|| {
            let tx = self.connection.transaction().map_err(failure)?;
            for ((item, bytes), path) in prepared.iter().zip(&paths) {
                let mut options = OpenOptions::new();
                options.write(true).create_new(true);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.mode(0o600);
                }
                let mut file = options.open(path).map_err(failure)?;
                created.push(path.clone());
                file.write_all(bytes)
                    .and_then(|_| file.sync_all())
                    .map_err(failure)?;
                tx.execute("INSERT INTO attachments (id, source, name, mime, byte_count, width, height, sha256, created_at_ms, owner_chat_id) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
                    params![item.id, source.value(), item.name, item.mime, item.bytes as i64, item.width, item.height, digest(bytes), item.created_at_ms, chat_id]).map_err(failure)?;
            }
            // Persist directory entries before committing metadata that references them.
            #[cfg(unix)]
            File::open(&self.directory)
                .and_then(|file| file.sync_all())
                .map_err(failure)?;
            tx.commit().map_err(failure)
        })();
        if let Err(error) = result {
            for path in created {
                let _ = fs::remove_file(path);
            }
            return Err(error);
        }
        Ok(prepared.into_iter().map(|(item, _)| item).collect())
    }
    pub fn remove(&mut self, source: Source, id: &str) -> Result<(), String> {
        self.remove_scoped(Some(source), None, id)
    }
    fn remove_scoped(
        &mut self,
        source: Option<Source>,
        chat_id: Option<&str>,
        id: &str,
    ) -> Result<(), String> {
        let found: Option<(String, String)> = self
            .connection
            .query_row(
                "SELECT source, mime FROM attachments WHERE id = ?1 AND owner_chat_id IS ?2 AND message_id IS NULL",
                params![id, chat_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(failure)?;
        let (owner, mime) = found.ok_or("Anexo não encontrado nesta coleção.")?;
        if source.is_some_and(|source| owner != source.value()) {
            return Err("Anexo não encontrado nesta coleção.".into());
        }
        let path = self.path(id, &mime)?;
        self.connection
            .execute("UPDATE attachments SET deleted = 1 WHERE id = ?1", [id])
            .map_err(failure)?;
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => {
                Err("Anexo desativado; a limpeza do arquivo será repetida ao reiniciar.".into())
            }
        }
    }
    pub fn resolve(&self, source: Source, ids: &[String]) -> Result<Vec<PathBuf>, String> {
        if ids.len() > MAX_ATTACHMENTS || ids.iter().collect::<HashSet<_>>().len() != ids.len() {
            return Err("Lista de anexos inválida.".into());
        }
        let rows = self.rows(source)?;
        ids.iter()
            .map(|id| {
                let (item, hash) = rows
                    .iter()
                    .find(|(item, _)| &item.id == id)
                    .ok_or("Anexo indisponível nesta coleção.")?;
                self.verified_bytes(item, hash)?;
                self.path(id, &item.mime)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
