mod microvm;
mod snapshot_manager;
mod fuse_bridge;
mod disk_streamer;

use tokio::net::UnixListener;
use crate::microvm::MicroVmController;

/// This daemon listens on a local Unix socket for IPC commands from Firefox.
/// When the browser intercepts a `javasandbox://` URI, it pings this daemon to
/// spin up a Firecracker MicroVM.

#[tokio::main]
async fn main() {
    println!("Bobzilla JavaSandbox Runner Daemon Initialized.");

    // Bind to the designated IPC socket for Firefox communication
    let socket_path = "/run/user/1000/bobzilla-sandbox.sock"; // Should dynamically resolve UID in production

    // Cleanup existing socket file if it exists
    let _ = std::fs::remove_file(socket_path);

    let listener = match UnixListener::bind(socket_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to IPC socket: {}", e);
            return;
        }
    };

    println!("Listening for Firefox hooks on {}...", socket_path);

    loop {
        match listener.accept().await {
            Ok((_stream, _addr)) => {
                println!("Received async connection from Gecko engine.");
                // TODO: Parse URI payload and validate cryptography

                tokio::spawn(async move {
                    launch_microvm("dummy-payload-hash").await;
                });
            }
            Err(e) => {
                eprintln!("IPC connection failed: {}", e);
            }
        }
    }
}

async fn launch_microvm(payload_hash: &str) {
    println!("Preparing to launch Firecracker MicroVM for payload: {}", payload_hash);

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
