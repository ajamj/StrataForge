use std::process::{Child, Command, Stdio, ChildStdin, ChildStdout};
use std::io::{BufReader, BufRead, Write};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use anyhow::{Result, anyhow};

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    id: u64,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

pub struct IpcBridge {
    inner: Arc<Mutex<Option<WorkerInstance>>>,
    next_id: Arc<Mutex<u64>>,
}

struct WorkerInstance {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl IpcBridge {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    fn ensure_worker(&self) -> Result<()> {
        let mut inner_guard = self.inner.lock().unwrap();
        
        if let Some(ref mut instance) = *inner_guard {
             if let Ok(None) = instance._child.try_wait() {
                 return Ok(());
             }
        }

        // Spawn worker
        // In a production environment, we'd use the actual binary path.
        // For development, we use 'cargo run'.
        let mut child = Command::new("cargo")
            .args(["run", "-p", "seisly_py_worker", "--quiet"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow!("Failed to spawn worker: {}", e))?;
        
        let stdin = child.stdin.take().ok_or_else(|| anyhow!("Stdin not available"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Stdout not available"))?;

        *inner_guard = Some(WorkerInstance {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        });
        
        Ok(())
    }

    pub fn execute(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        self.ensure_worker()?;
        
        let id = {
            let mut id_guard = self.next_id.lock().unwrap();
            let id = *id_guard;
            *id_guard += 1;
            id
        };

        let req = Request {
            id,
            method: method.to_string(),
            params,
        };

        let result = self.execute_internal(req);
        
        if result.is_err() {
            // Communication error or worker crash, clear instance to force restart next time
            let mut inner_guard = self.inner.lock().unwrap();
            *inner_guard = None;
        }
        
        result
    }

    fn execute_internal(&self, req: Request) -> Result<serde_json::Value> {
        let mut inner_guard = self.inner.lock().unwrap();
        let instance = inner_guard.as_mut().ok_or_else(|| anyhow!("Worker not initialized"))?;
        
        let req_json = serde_json::to_string(&req)? + "\n";
        instance.stdin.write_all(req_json.as_bytes())?;
        instance.stdin.flush()?;

        let mut line = String::new();
        instance.stdout.read_line(&mut line)
            .map_err(|e| anyhow!("Failed to read from worker: {}", e))?;

        if line.is_empty() {
            return Err(anyhow!("Worker closed output pipe (EOF)"));
        }

        let resp: Response = serde_json::from_str(&line)
            .map_err(|e| anyhow!("Failed to parse response '{}': {}", line, e))?;

        if let Some(err) = resp.error {
            return Err(anyhow!("Worker error: {}", err));
        }

        resp.result.ok_or_else(|| anyhow!("Missing result in response"))
    }
}

impl Default for IpcBridge {
    fn default() -> Self {
        Self::new()
    }
}
