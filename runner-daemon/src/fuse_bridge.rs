use std::sync::Arc;
use tokio::sync::Mutex;
use crate::disk_streamer::HttpBlockDevice;

/// MOCK: FUSE (Filesystem in Userspace) Bridge
/// This module is responsible for taking the `HttpBlockDevice` (which fetches blocks over the internet)
/// and exposing it as a local block device (e.g., `/dev/vda_javasandbox`) on the host operating system.
/// Firecracker mounts this local block device, streaming disk reads via HTTP Range Requests in the background.
pub struct JavasandboxFuse {
    pub block_device: Arc<Mutex<HttpBlockDevice>>,
    pub mount_point: String,
}

impl JavasandboxFuse {
    pub fn new(device: HttpBlockDevice, mount: &str) -> Self {
        Self {
            block_device: Arc::new(Mutex::new(device)),
            mount_point: mount.to_string(),
        }
    }

    pub fn mount(&self) {
        println!("Mounting HTTP Block Device to FUSE path: {}", self.mount_point);
        println!("FUSE Mount successful. Firecracker can now boot from this path.");
    }

    pub fn unmount(&self) {
        println!("Unmounting FUSE path: {}", self.mount_point);
    }
}
