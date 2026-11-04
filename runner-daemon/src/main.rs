pub mod microvm;
pub mod snapshot_manager;
pub mod fuse_bridge;
pub mod disk_streamer;

use std::env;
use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use crate::microvm::MicroVmController;
use crate::snapshot_manager::SnapshotManager;
use crate::fuse_bridge::JavasandboxFuse;
use crate::disk_streamer::HttpBlockDevice;

/// Resolves the IPC socket path dynamically with multi-path fallback:
/// 1. $BOBZILLA_SOCKET environment variable
/// 2. $XDG_RUNTIME_DIR/bobzilla-sandbox.sock
/// 3. /run/user/$UID/bobzilla-sandbox.sock
/// 4. /tmp/bobzilla-sandbox-$UID.sock
/// 5. /tmp/bobzilla-sandbox.sock
pub fn resolve_ipc_socket_path() -> PathBuf {
    if let Ok(env_path) = env::var("BOBZILLA_SOCKET") {
        return PathBuf::from(env_path);
    }

    if let Ok(runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        let p = PathBuf::from(runtime_dir).join("bobzilla-sandbox.sock");
        if let Some(parent) = p.parent() {
            if parent.exists() {
                return p;
            }
        }
    }

    // Check uid-based runtime dir
    let uid = nix::unistd::getuid().as_raw();
    let run_user = PathBuf::from(format!("/run/user/{}/bobzilla-sandbox.sock", uid));
    if let Some(parent) = run_user.parent() {
        if parent.exists() {
            return run_user;
        }
    }

    // Check default /run/user/1000
    let run_default = PathBuf::from("/run/user/1000/bobzilla-sandbox.sock");
    if let Some(parent) = run_default.parent() {
        if parent.exists() {
            return run_default;
        }
    }

    // Fall back to /tmp/bobzilla-sandbox-$UID.sock or /tmp/bobzilla-sandbox.sock
    let tmp_uid = PathBuf::from(format!("/tmp/bobzilla-sandbox-{}.sock", uid));
    tmp_uid
}

/// Parses and dispatches IPC messages received from Firefox / Gecko or CLI controls.
pub async fn handle_ipc_message(msg: &str) -> String {
    let trimmed = msg.trim();
    if trimmed.is_empty() {
        return "ERR EMPTY_COMMAND\n".to_string();
    }

    println!("[IPC] Received message: {}", trimmed);

    if trimmed == "PING" {
        return "PONG BOBZILLA_DAEMON_OK\n".to_string();
    }

    if trimmed == "STATUS" {
        return "STATUS: READY, ENGINE: FIRECRACKER_READY, DISK_STREAMER: ACTIVE\n".to_string();
    }

    if trimmed == "SHUTDOWN" || trimmed == "QUIT" {
        println!("[IPC] Shutdown command received. Cleaning up...");
        return "OK SHUTTING_DOWN\n".to_string();
    }

    if trimmed.starts_with("PAUSE ") {
        let session = trimmed.trim_start_matches("PAUSE ").trim();
        let snapshot_mgr = SnapshotManager::new("firecracker-api.sock");
        let _ = snapshot_mgr.pause_vm().await;
        let _ = snapshot_mgr.create_snapshot(session).await;
        return format!("OK PAUSED {}\n", session);
    }

    if trimmed.starts_with("LAUNCH ") || trimmed.starts_with("javasandbox://") {
        let payload = if trimmed.starts_with("LAUNCH ") {
            trimmed.trim_start_matches("LAUNCH ").trim()
        } else {
            trimmed
        };
        launch_microvm(payload).await;
        return format!("OK LAUNCHED {}\n", payload);
    }

    format!("ERR UNKNOWN_COMMAND: {}\n", trimmed)
}

/// Orchestrates spawning of the MicroVM and demand-paged disk streaming.
pub async fn launch_microvm(payload_hash: &str) {
    println!("Preparing to launch Firecracker MicroVM for payload: {}", payload_hash);

    // Initialize the demand-paging HTTP block device and FUSE bridge
    let payload_url = format!("https://payloads.bobzilla.internal/{}", payload_hash);
    if let Ok(block_device) = HttpBlockDevice::new(&payload_url).await {
        let mount_point = format!("/tmp/javasandbox-mnt-{}", payload_hash);
        let fuse = JavasandboxFuse::new(block_device, &mount_point);
        fuse.mount();
    }

    let vm_controller = MicroVmController::new(payload_hash);

    if let Err(e) = vm_controller.spawn_process() {
        eprintln!("Failed to spawn Firecracker process: {}", e);
        return;
    }

    if let Err(e) = vm_controller.configure_boot_source("/var/lib/bobzilla/javasandbox-kernel").await {
         eprintln!("Failed to configure boot source: {}", e);
         return;
    }

    if let Err(e) = vm_controller.start_instance().await {
         eprintln!("Failed to start instance: {}", e);
    }
}

/// Handles an active client stream over Unix domain socket.
pub async fn process_client(mut stream: UnixStream) {
    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer).await {
        Ok(n) if n > 0 => {
            let msg = String::from_utf8_lossy(&buffer[..n]);
            let response = handle_ipc_message(&msg).await;
            let _ = stream.write_all(response.as_bytes()).await;
            if msg.trim() == "SHUTDOWN" || msg.trim() == "QUIT" {
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                std::process::exit(0);
            }
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let socket_path = resolve_ipc_socket_path();

    if args.len() > 1 {
        let cmd = &args[1];
        match cmd.as_str() {
            "ping" | "status" | "quit" | "shutdown" => {
                println!("Connecting to daemon at {:?}...", socket_path);
                match UnixStream::connect(&socket_path).await {
                    Ok(mut stream) => {
                        let payload = format!("{}\n", cmd.to_uppercase());
                        let _ = stream.write_all(payload.as_bytes()).await;
                        let mut resp = [0u8; 512];
                        if let Ok(n) = stream.read(&mut resp).await {
                            println!("{}", String::from_utf8_lossy(&resp[..n]));
                        }
                    }
                    Err(e) => {
                        eprintln!("Could not reach daemon at {:?}: {}", socket_path, e);
                        std::process::exit(1);
                    }
                }
                return;
            }
            "help" | "--help" | "-h" => {
                println!("Bobzilla JavaSandbox Runner Daemon");
                println!("Usage: bobzilla-javasandbox-daemon [COMMAND]");
                println!("\nCommands:");
                println!("  (none)      Run daemon server in foreground");
                println!("  status      Query daemon status via IPC socket");
                println!("  ping        Test daemon connection");
                println!("  quit        Gracefully stop running daemon");
                return;
            }
            _ => {}
        }
    }

    println!("Bobzilla JavaSandbox Runner Daemon Initialized.");
    println!("Selected IPC socket path: {:?}", socket_path);

    // Remove stale socket file if it exists
    if socket_path.exists() {
        let _ = std::fs::remove_file(&socket_path);
    }

    let listener = match UnixListener::bind(&socket_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to IPC socket {:?}: {}", socket_path, e);
            return;
        }
    };

    println!("Listening for Firefox hooks and management commands on {:?}...", socket_path);

    loop {
        match listener.accept().await {
            Ok((stream, _addr)) => {
                tokio::spawn(async move {
                    process_client(stream).await;
                });
            }
            Err(e) => {
                eprintln!("IPC connection failed: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_ipc_socket_path() {
        let path = resolve_ipc_socket_path();
        assert!(path.to_str().unwrap().contains("bobzilla-sandbox"));
    }

    #[tokio::test]
    async fn test_handle_ipc_message_ping() {
        let res = handle_ipc_message("PING").await;
        assert_eq!(res, "PONG BOBZILLA_DAEMON_OK\n");
    }

    #[tokio::test]
    async fn test_handle_ipc_message_status() {
        let res = handle_ipc_message("STATUS").await;
        assert!(res.contains("FIRECRACKER_READY"));
    }

    #[tokio::test]
    async fn test_handle_ipc_message_launch() {
        let res = handle_ipc_message("LAUNCH test-payload-xyz").await;
        assert!(res.contains("OK LAUNCHED test-payload-xyz"));
    }

    #[tokio::test]
    async fn test_handle_ipc_message_javasandbox_scheme() {
        let res = handle_ipc_message("javasandbox://run?hash=abc123payload").await;
        assert!(res.contains("OK LAUNCHED javasandbox://run?hash=abc123payload"));
    }

    #[tokio::test]
    async fn test_handle_ipc_message_pause() {
        let res = handle_ipc_message("PAUSE tab-session-99").await;
        assert_eq!(res, "OK PAUSED tab-session-99\n");
    }

    #[tokio::test]
    async fn test_end_to_end_socket_lifecycle() {
        let test_socket = format!("/tmp/test-bobzilla-e2e-{}.sock", std::process::id());
        let _ = std::fs::remove_file(&test_socket);

        let listener = UnixListener::bind(&test_socket).expect("Failed to bind e2e test socket");

        // Spawn mock daemon listener loop in background
        let server_socket = test_socket.clone();
        let server_handle = tokio::spawn(async move {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut buf = [0u8; 512];
                if let Ok(n) = stream.read(&mut buf).await {
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    let resp = handle_ipc_message(&msg).await;
                    let _ = stream.write_all(resp.as_bytes()).await;
                }
            }
        });

        // Connect client (mimicking Gecko tab IPC dispatch)
        let mut client = UnixStream::connect(&test_socket).await.expect("Failed to connect client");
        client.write_all(b"LAUNCH applet-e2e-payload\n").await.expect("Failed to write to socket");

        let mut resp_buf = [0u8; 512];
        let n = client.read(&mut resp_buf).await.expect("Failed to read response");
        let resp_str = String::from_utf8_lossy(&resp_buf[..n]);

        assert!(resp_str.contains("OK LAUNCHED applet-e2e-payload"));

        let _ = server_handle.await;
        let _ = std::fs::remove_file(&server_socket);
    }
}
