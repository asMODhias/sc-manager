use thiserror::Error;
use serde::Deserialize;
use sha3::Digest;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use crate::signature;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Signature verification failed")]
    SignatureFailed,

    #[error("Network error: {0}")]
    Network(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Hash mismatch")]
    HashMismatch,
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub signature_hex: String,
    pub public_hex: String,
    pub payload_url: String,
    pub payload_sha3_hex: String,
}

/// Minimal client that downloads manifests and payloads and verifies them.
pub struct ClientService {
    client: reqwest::blocking::Client,
}

impl ClientService {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder().build().unwrap();
        Self { client }
    }

    /// Download manifest JSON from `url` and deserialize
    pub fn fetch_manifest(&self, url: &str) -> Result<Manifest, ClientError> {
        let res = self
            .client
            .get(url)
            .send()
            .map_err(|e| ClientError::Network(e.to_string()))?;
        let manifest: Manifest = res.json().map_err(|e| ClientError::Network(e.to_string()))?;
        Ok(manifest)
    }

    /// Verify manifest signature (ed25519)
    pub fn verify_manifest(&self, manifest: &Manifest) -> Result<bool, ClientError> {
        let pubk = hex::decode(&manifest.public_hex).map_err(|_| ClientError::SignatureFailed)?;
        signature::verify_signature_ed25519(&manifest.version, &hex::decode(&manifest.signature_hex).map_err(|_| ClientError::SignatureFailed)?, &pubk).map_err(|_| ClientError::SignatureFailed)
    }

    /// Convenience: verify raw manifest version + signature + public key
    pub fn verify_manifest_ed25519(&self, version: &str, signature_hex: &str, public: &[u8]) -> Result<bool, ClientError> {
        let sig = hex::decode(signature_hex).map_err(|_| ClientError::SignatureFailed)?;
        signature::verify_signature_ed25519(version, &sig, public).map_err(|_| ClientError::SignatureFailed)
    }

    /// Download payload from `url` into `dest` and verify SHA3-256 against expected hex.
    /// Streams body to file and computes hash while writing.
    pub fn download_and_verify<P: AsRef<Path>>(&self, url: &str, dest: P, expected_sha3_hex: &str) -> Result<(), ClientError> {
        let res = self.client.get(url).send().map_err(|e| ClientError::Network(e.to_string()))?;
        let bytes = res.bytes().map_err(|e| ClientError::Network(e.to_string()))?;
        let mut file = File::create(dest.as_ref())?;
        file.write_all(&bytes)?;
        let mut hasher = sha3::Sha3_256::new();
        hasher.update(&bytes);
        let digest = hasher.finalize();
        let got = hex::encode(digest);
        if got != expected_sha3_hex {
            return Err(ClientError::HashMismatch);
        }
        Ok(())
    }
}

impl Default for ClientService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{MockServer, Method::GET};
    use sha3::{Digest, Sha3_256};
    use tempfile::NamedTempFile;

    #[test]
    fn fetch_and_verify_manifest_and_payload() {
        let server = MockServer::start();

        // prepare payload
        let payload = vec![1u8; 4096];
        let mut hasher = Sha3_256::new();
        hasher.update(&payload);
        let payload_hash = hex::encode(hasher.finalize());

        // create manifest using an ed25519 keypair
        let (pubk, seck) = crate::signature::generate_keypair();
        let version = "1.2.3".to_string();
        let sig = crate::signature::sign_with_keypair(&version, &seck, &pubk).unwrap();
        let sig_hex = hex::encode(&sig);
        let pub_hex = hex::encode(&pubk);

        // mock payload endpoint
        let payload_path = "/payload.bin";
        server.mock(|when, then| {
            when.method(GET).path(payload_path);
            then.status(200).body(payload.clone());
        });

        // mock manifest endpoint
        let manifest_path = "/manifest.json";
        let payload_url = format!("{}{}", server.base_url(), payload_path);
        let manifest = serde_json::json!({
            "version": version,
            "signature_hex": sig_hex,
            "public_hex": pub_hex,
            "payload_url": payload_url,
            "payload_sha3_hex": payload_hash
        });
        server.mock(|when, then| {
            when.method(GET).path(manifest_path);
            then.status(200).header("content-type", "application/json").body(manifest.to_string());
        });

        let client = ClientService::new();
        let manifest_url = format!("{}{}", server.base_url(), manifest_path);
        let m = client.fetch_manifest(&manifest_url).unwrap();
        assert!(client.verify_manifest(&m).unwrap());

        let tmp = NamedTempFile::new().unwrap();
        client.download_and_verify(&m.payload_url, tmp.path(), &m.payload_sha3_hex).unwrap();
    }
}
