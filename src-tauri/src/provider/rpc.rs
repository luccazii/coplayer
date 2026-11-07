use serde_json::{Value, json};
use std::{
    collections::HashMap,
    path::Path,
    process::Stdio,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, Command},
    sync::{Mutex, mpsc, oneshot},
    time::timeout,
};

type Reply = Result<Value, String>;
const MAX_LINE: u64 = 2 * 1024 * 1024;

pub struct Rpc {
    input: Mutex<ChildStdin>,
    child: Mutex<Child>,
    pending: Mutex<HashMap<u64, oneshot::Sender<Reply>>>,
    sequence: AtomicU64,
}

impl Rpc {
    pub fn spawn(
        binary: &Path,
        home: &Path,
        cwd: &Path,
    ) -> Result<(Arc<Self>, mpsc::Receiver<Value>), String> {
        let mut command = Command::new(binary);
        command
            .args(["app-server", "--listen", "stdio://"])
            .env_clear()
            .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
            .env("CODEX_HOME", home)
            .env("LANG", "en_US.UTF-8")
            .current_dir(cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        // macOS APIs need the user's real home; CODEX_HOME alone owns credentials
        // and config. No API keys, parent Codex env, or shell startup are inherited.
        if let Some(home) = std::env::var_os("HOME") {
            command.env("HOME", home);
        }
        if let Some(tmp) = std::env::var_os("TMPDIR") {
            command.env("TMPDIR", tmp);
        }
        let mut child = command
            .spawn()
            .map_err(|_| "Não foi possível iniciar o runtime empacotado.".to_string())?;
        let input = child.stdin.take().ok_or("Runtime sem entrada padrão.")?;
        let output = child.stdout.take().ok_or("Runtime sem saída padrão.")?;
        let rpc = Arc::new(Self {
            input: Mutex::new(input),
            child: Mutex::new(child),
            pending: Mutex::new(HashMap::new()),
            sequence: AtomicU64::new(1),
        });
        let weak = Arc::downgrade(&rpc);
        let (events, receiver) = mpsc::channel(128);
        tokio::spawn(async move {
            let mut reader = BufReader::new(output);
            loop {
                let mut line = Vec::new();
                let read = (&mut reader)
                    .take(MAX_LINE + 1)
                    .read_until(b'\n', &mut line)
                    .await;
                match read {
                    Ok(0) | Err(_) => break,
                    Ok(_) if line.len() as u64 > MAX_LINE => break,
                    Ok(_) => {}
                }
                let Ok(message) = serde_json::from_slice::<Value>(&line) else {
                    break;
                };
                let Some(rpc) = weak.upgrade() else { return };
                if message.get("method").is_some() && message.get("id").is_some() {
                    // No approvals, tool execution, token delegation or elicitation
                    // are supported by this consultative proof.
                    let response = json!({"id":message["id"],"error":{"code":-32601,"message":"Coplayer proof does not allow runtime tools or approval requests"}});
                    let _ = rpc.write(&response).await;
                    let _ = events.send(json!({"method":"coplayer/toolRejected"})).await;
                } else if let Some(id) = message.get("id").and_then(Value::as_u64) {
                    if let Some(sender) = rpc.pending.lock().await.remove(&id) {
                        let result = if message.get("error").is_some() {
                            Err("O runtime recusou a operação. Verifique autenticação, limites e compatibilidade da versão.".into())
                        } else {
                            Ok(message["result"].clone())
                        };
                        let _ = sender.send(result);
                    }
                } else if events.send(message).await.is_err() {
                    break;
                }
            }
            if let Some(rpc) = weak.upgrade() {
                for (_, sender) in rpc.pending.lock().await.drain() {
                    let _ = sender.send(Err("A conexão com o runtime foi encerrada.".into()));
                }
                let _ = rpc.child.lock().await.kill().await;
            }
            let _ = events.send(json!({"method":"coplayer/disconnected"})).await;
        });
        Ok((rpc, receiver))
    }

    async fn write(&self, message: &Value) -> Result<(), String> {
        let mut bytes =
            serde_json::to_vec(message).map_err(|_| "Mensagem inválida.".to_string())?;
        bytes.push(b'\n');
        let mut input = self.input.lock().await;
        input
            .write_all(&bytes)
            .await
            .map_err(|_| "Runtime desconectado.".to_string())?;
        input
            .flush()
            .await
            .map_err(|_| "Runtime desconectado.".to_string())
    }

    pub async fn request(&self, method: &str, params: Value) -> Reply {
        let id = self.sequence.fetch_add(1, Ordering::Relaxed);
        let (sender, receiver) = oneshot::channel();
        self.pending.lock().await.insert(id, sender);
        let result = timeout(Duration::from_secs(40), async {
            self.write(&json!({"id":id,"method":method,"params":params}))
                .await?;
            receiver
                .await
                .map_err(|_| "Runtime desconectado.".to_string())?
        })
        .await
        .unwrap_or_else(|_| {
            Err("O runtime não respondeu a tempo. Encerre e reinicie para tentar novamente.".into())
        });
        self.pending.lock().await.remove(&id);
        result
    }

    pub async fn notify(&self, method: &str) -> Result<(), String> {
        self.write(&json!({"method":method})).await
    }

    pub async fn stop(&self) {
        let mut child = self.child.lock().await;
        let _ = child.kill().await;
        let _ = timeout(Duration::from_secs(3), child.wait()).await;
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn fixture(script: &str) -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(super::super::identifier("coplayer-rpc-test"));
        std::fs::create_dir_all(&directory).unwrap();
        let binary = directory.join("runtime");
        std::fs::write(&binary, format!("#!/bin/sh\n{script}\n")).unwrap();
        std::fs::set_permissions(&binary, std::fs::Permissions::from_mode(0o700)).unwrap();
        directory
    }

    #[tokio::test]
    async fn runtime_eof_rejects_waiting_request() {
        let directory = fixture("IFS= read -r line\nexit 0");
        let (rpc, mut events) =
            Rpc::spawn(&directory.join("runtime"), &directory, &directory).unwrap();
        let result = timeout(Duration::from_secs(3), rpc.request("probe", json!({})))
            .await
            .unwrap();
        assert!(result.is_err());
        assert_eq!(
            events.recv().await.unwrap()["method"],
            "coplayer/disconnected"
        );
        rpc.stop().await;
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[tokio::test]
    async fn runtime_tool_requests_are_rejected_instead_of_approved() {
        let directory = fixture(
            r#"IFS= read -r line
printf '%s\n' '{"id":"tool-1","method":"item/commandExecution/requestApproval","params":{}}'
IFS= read -r line
case "$line" in
  *'"code":-32601'*) printf '%s\n' '{"id":1,"result":{"denied":true}}' ;;
  *) printf '%s\n' '{"id":1,"result":{"denied":false}}' ;;
esac
IFS= read -r line"#,
        );
        let (rpc, mut events) =
            Rpc::spawn(&directory.join("runtime"), &directory, &directory).unwrap();
        let result = timeout(Duration::from_secs(3), rpc.request("probe", json!({})))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result["denied"], true);
        assert_eq!(
            events.recv().await.unwrap()["method"],
            "coplayer/toolRejected"
        );
        rpc.stop().await;
        std::fs::remove_dir_all(directory).unwrap();
    }
}
