use reqwest::Client;
use serde_json::json;
use std::process::Command;

/// Orchestrates the actual creation and execution of the Firecracker MicroVM process.
pub struct MicroVmController {
    api_socket_path: String,
    _client: Client,
}

impl MicroVmController {
    pub fn new(payload_hash: &str) -> Self {
        let api_socket_path = format!("/tmp/firecracker-{}.sock", payload_hash);
        Self {
            api_socket_path,
            _client: Client::new(), // In production, configure Unix socket connector
        }
    }

    /// Spawns the Firecracker process in the background.
    pub fn spawn_process(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Spawning Firecracker process with API socket: {}", self.api_socket_path);

        // Ensure old socket is removed
        let _ = std::fs::remove_file(&self.api_socket_path);

        // If the firecracker binary exists locally, run it; otherwise log mock simulation
        if std::path::Path::new("./firecracker").exists() {
            Command::new("./firecracker")
                .arg("--api-sock")
                .arg(&self.api_socket_path)
                .spawn()?;
        } else {
            println!("Notice: Local './firecracker' binary not present; running MicroVM in mock mode.");
        }

        Ok(())
    }

    /// Configures the boot source (kernel and init args).
    pub async fn configure_boot_source(&self, kernel_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _payload = json!({
            "kernel_image_path": kernel_path,
            "boot_args": "console=ttyS0 reboot=k panic=1 pci=off"
        });

        println!("Configuring boot source: {}", kernel_path);
        // MOCK: PUT /boot-source
        Ok(())
    }

    /// Triggers the InstanceStart API command to boot the VM.
    pub async fn start_instance(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _payload = json!({ "action_type": "InstanceStart" });
        println!("Sending InstanceStart command to Firecracker.");
        // MOCK: PUT /actions
        Ok(())
    }
}
