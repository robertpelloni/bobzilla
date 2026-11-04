use reqwest::Client;
use serde_json::json;

/// MOCK: Firecracker Snapshot Manager
/// This module intercepts Firefox's tab close events and pauses the Firecracker MicroVM.
/// It uses Firecracker's REST API to serialize the guest memory and vCPU state
/// to disk, allowing instant sub-100ms resume times.
pub struct SnapshotManager {
    pub firecracker_socket_url: String,
    pub client: Client,
}

impl SnapshotManager {
    pub fn new(socket_path: &str) -> Self {
        Self {
            firecracker_socket_url: format!("http://localhost/{}", socket_path),
            client: Client::new(),
        }
    }

    /// Instructs Firecracker to pause execution instantly.
    pub async fn pause_vm(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sending PAUSE command to Firecracker via API socket: {}", self.firecracker_socket_url);
        let _payload = json!({ "state": "Paused" });
        Ok(())
    }

    /// Triggers Firecracker to dump RAM and CPU registers to disk.
    pub async fn create_snapshot(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Creating diff snapshot for session: {}", session_id);

        let mem_file = format!("./{}.mem", session_id);
        let snapshot_file = format!("./{}.vif", session_id);

        let _payload = json!({
            "snapshot_type": "Diff",
            "mem_file_path": mem_file,
            "snapshot_path": snapshot_file
        });

        println!("Memory state successfully serialized to disk ({}, {}).", mem_file, snapshot_file);
        Ok(())
    }
}
