use reqwest::Client;

/// S3/MinIO object storage abstraction for encrypted media blobs.
#[derive(Clone)]
pub struct MediaStorage {
    client: Client,
    endpoint: String,
    bucket: String,
    _access_key: String,
    _secret_key: String,
}

impl MediaStorage {
    pub fn new(endpoint: String, bucket: String, access_key: String, secret_key: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            bucket,
            _access_key: access_key,
            _secret_key: secret_key,
        }
    }

    /// Upload encrypted blob to S3/MinIO. Returns the object key.
    pub async fn upload(&self, key: &str, data: &[u8]) -> anyhow::Result<()> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let response = self
            .client
            .put(&url)
            .header("Content-Type", "application/octet-stream")
            .body(data.to_vec())
            .send()
            .await?;
        if !response.status().is_success() {
            anyhow::bail!("Upload failed with status: {}", response.status());
        }
        Ok(())
    }

    /// Download encrypted blob from S3/MinIO.
    pub async fn download(&self, key: &str) -> anyhow::Result<Vec<u8>> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }
        let data = response.bytes().await?.to_vec();
        Ok(data)
    }

    /// Delete encrypted blob from S3/MinIO.
    pub async fn delete(&self, key: &str) -> anyhow::Result<()> {
        let url = format!("{}/{}/{}", self.endpoint, self.bucket, key);
        let client = Client::new();
        let response = client.delete(&url).send().await?;
        if !response.status().is_success() {
            anyhow::bail!("Delete failed with status: {}", response.status());
        }
        Ok(())
    }
}
