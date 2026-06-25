use std::sync::Arc;
use reqwest::Client;

/// Handles the demand-paging HTTP range-request logic for the Javasandbox.
/// Instead of downloading the 150MB+ Java application and guest OS image fully upfront,
/// this struct intercepts the Firecracker VirtIO block device requests and streams
/// the exact 4KB blocks needed directly from the server over HTTPS.

pub struct HttpBlockDevice {
    client: Arc<Client>,
    payload_url: String,
    total_size: u64,
}

impl HttpBlockDevice {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Arc::new(Client::new());

        // Fetch headers to determine the total size of the payload image
        let head_resp = client.head(url).send().await?;
        let total_size = head_resp.content_length().unwrap_or(0);

        if total_size == 0 {
            return Err("Payload URL does not return Content-Length. Streaming impossible.".into());
        }

        Ok(Self {
            client,
            payload_url: url.to_string(),
            total_size,
        })
    }

    /// Fetches a specific chunk (e.g., a 4KB sector) requested by the guest OS kernel
    pub async fn read_sector(&self, offset: u64, size: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let end = offset + size as u64 - 1;
        let range_header = format!("bytes={}-{}", offset, end);

        let resp = self.client.get(&self.payload_url)
            .header("Range", range_header)
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(format!("Failed to fetch chunk. Status: {}", resp.status()).into());
        }

        let bytes = resp.bytes().await?;
        Ok(bytes.to_vec())
    }
}
