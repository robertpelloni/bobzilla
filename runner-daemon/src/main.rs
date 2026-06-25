mod fuse_bridge;

mod disk_streamer;

use tokio::net::UnixListener;

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

                // MOCK: Launch Firecracker process
                launch_microvm("dummy-payload-hash");
            }
            Err(e) => {
                eprintln!("IPC connection failed: {}", e);
            }
        }
    }
}

fn launch_microvm(payload_hash: &str) {
    println!("Preparing to launch Firecracker MicroVM for payload: {}", payload_hash);

    // Spawn the Firecracker process
    // Command::new("./firecracker")
    //     .arg("--api-sock")
    //     .arg(format!("/tmp/firecracker-{}.sock", payload_hash))
    //     .spawn()
    //     .expect("Failed to spawn Firecracker");
}
