use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct RpcResponse {
    jsonrpc: &'static str,
    result: serde_json::Value,
    id: serde_json::Value,
}

#[tokio::main]
async fn main() {
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let mut stdout = tokio::io::stdout();

    let mut counter: u8 = 0;

    while let Ok(Some(line)) = lines.next_line().await {
        let req: RpcRequest = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if req.method == "say_hello" {
            let name = req
                .params
                .as_ref()
                .and_then(|p| p.get(0))
                .and_then(|v| v.as_str())
                .unwrap_or("world");

            let response = RpcResponse {
                jsonrpc: "2.0",
                result: json!(format!("Hello, {name} {counter}")),
                id: req.id.unwrap_or(json!(null)),
            };

            counter += 1;

            let resp = serde_json::to_string(&response).unwrap();
            stdout.write_all(resp.as_bytes()).await.unwrap();
            stdout.write_all(b"\n").await.unwrap();
            stdout.flush().await.unwrap();
        }
    }
}
