---
title: COPILOT_MASTER_INSTRUCTION_V8.0_FINAL
version: 8.0.0
date: 2025-12-30
status: ABSOLUTE_BINDING
priority: CRITICAL
applies_to: ALL_CODE_GENERATION
---

# 🤖 COPILOT MASTER INSTRUCTION V8.0 - ABSOLUTE & BINDING

**Zero Deviation | Zero Questions | Maximum Execution Efficiency**

---

## 🎯 INSTRUCTION HIERARCHY

```yaml
Priority_Order:
  1. CIG_ToS_Compliance (Star Citizen EULA) ← HIGHEST
  2. Security_Rules (no vulnerabilities)
  3. Architecture_Rules (clean code)
  4. Performance_Rules (optimized)
  5. Testing_Rules (coverage)
  6. Documentation_Rules (clarity)

Conflict_Resolution:
  IF conflict BETWEEN rules:
    - CIG ToS wins ALWAYS
    - Security over performance
    - Architecture over convenience
    - Tests over speed
    
  IF unsure:
    - Use most conservative approach
    - Default to LOCAL + P2P (never cloud)
    - Generate code immediately
    - Let CI/tests validate
```

---

## 🚀 PART 1: PROGRAM UPDATE LOGIC (FIRST PRIORITY)

### 1.1 Update System Architecture

```yaml
Update_Philosophy:
  - P2P-First (no central distribution server)
  - Author-Signed (cryptographic verification)
  - Delta-Based (bandwidth efficient)
  - Rollback-Safe (automatic on failure)
  - Zero-Downtime (rolling updates in mesh)

Update_Flow:
  1. Author signs update → Master Server
  2. Master publishes metadata (hash, signature)
  3. Clients detect new version
  4. Clients initiate P2P download
  5. Clients verify signature
  6. Clients apply delta patch
  7. Clients report success/failure
  8. Auto-rollback on failure
```

### 1.2 Implementation (FIRST CODE TO GENERATE)

```rust
// infrastructure/update-system/src/lib.rs

//! SC Manager Update System
//! 
//! P2P-based, author-signed, delta update distribution.
//! 
//! Architecture:
//! - Author Master Server: Signs and publishes updates
//! - P2P Mesh: Distributes update chunks
//! - Local Client: Verifies and applies updates
//! - Rollback: Automatic on failure

pub mod author;
pub mod client;
pub mod delta;
pub mod signature;
pub mod rollback;

use chrono::{DateTime, Utc};
use ed25519_dalek::{PublicKey, Signature};
use semver::Version;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_512};

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// UPDATE MANIFEST
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Update Manifest
/// 
/// Published by Author Master Server after signing.
/// Distributed via P2P mesh as metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateManifest {
    /// Version being updated to
    pub version: Version,
    
    /// Build date
    pub build_date: DateTime<Utc>,
    
    /// Release channel
    pub channel: ReleaseChannel,
    
    /// Update type
    pub update_type: UpdateType,
    
    /// Minimum version required to apply this update
    pub min_version: Version,
    
    /// Content hash (SHA3-512)
    pub content_hash: String,
    
    /// Author signature (Ed25519)
    pub signature: String,
    
    /// File list with hashes
    pub files: Vec<FileEntry>,
    
    /// Total size (bytes)
    pub total_size: u64,
    
    /// Changelog URL
    pub changelog_url: String,
    
    /// Is this a required update?
    pub required: bool,
    
    /// Rollback configuration
    pub rollback: RollbackConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReleaseChannel {
    Alpha,
    Beta,
    ReleaseCandidate,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Hotfix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Relative path from installation root
    pub path: String,
    
    /// File hash (SHA3-256)
    pub hash: String,
    
    /// File size (bytes)
    pub size: u64,
    
    /// Content-addressed identifier (IPFS CID)
    pub cid: String,
    
    /// Is this a delta patch?
    pub delta: bool,
    
    /// If delta, base version
    pub base_version: Option<Version>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// Automatically rollback on failure
    pub automatic: bool,
    
    /// Conditions that trigger rollback
    pub conditions: Vec<RollbackCondition>,
    
    /// Timeout before considering update failed (seconds)
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackCondition {
    /// Plugin load failure rate
    PluginLoadFailureRate(f32),
    
    /// Crash on startup
    CrashOnStartup,
    
    /// Critical error rate
    CriticalErrorRate(f32),
    
    /// P2P mesh disconnection
    MeshDisconnected,
}

impl UpdateManifest {
    /// Create new update manifest
    pub fn new(
        version: Version,
        channel: ReleaseChannel,
        update_type: UpdateType,
        min_version: Version,
        files: Vec<FileEntry>,
        changelog_url: String,
        required: bool,
    ) -> Self {
        let total_size = files.iter().map(|f| f.size).sum();
        
        let content_hash = Self::calculate_content_hash(&files);
        
        Self {
            version,
            build_date: Utc::now(),
            channel,
            update_type,
            min_version,
            content_hash,
            signature: String::new(), // Will be set by author
            files,
            total_size,
            changelog_url,
            required,
            rollback: RollbackConfig::default(),
        }
    }
    
    /// Sign manifest with author key
    pub fn sign(&mut self, author_key: &ed25519_dalek::Keypair) -> Result<(), UpdateError> {
        let data = self.signing_data();
        let signature = author_key.sign(data.as_bytes());
        self.signature = hex::encode(signature.to_bytes());
        Ok(())
    }
    
    /// Verify manifest signature
    pub fn verify(&self, author_pubkey: &PublicKey) -> Result<bool, UpdateError> {
        let signature_bytes = hex::decode(&self.signature)
            .map_err(|e| UpdateError::SignatureInvalid(e.to_string()))?;
        
        let signature = Signature::from_bytes(&signature_bytes)
            .map_err(|e| UpdateError::SignatureInvalid(e.to_string()))?;
        
        let data = self.signing_data();
        
        Ok(author_pubkey.verify(data.as_bytes(), &signature).is_ok())
    }
    
    fn signing_data(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.version,
            self.content_hash,
            self.build_date.to_rfc3339(),
            serde_json::to_string(&self.files).unwrap()
        )
    }
    
    fn calculate_content_hash(files: &[FileEntry]) -> String {
        let mut hasher = Sha3_512::new();
        
        for file in files {
            hasher.update(file.path.as_bytes());
            hasher.update(file.hash.as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            automatic: true,
            conditions: vec![
                RollbackCondition::PluginLoadFailureRate(0.5),
                RollbackCondition::CrashOnStartup,
                RollbackCondition::CriticalErrorRate(0.1),
            ],
            timeout_seconds: 300, // 5 minutes
        }
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// UPDATE CLIENT
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Update Client
/// 
/// Manages update checking, downloading, and applying.
pub struct UpdateClient {
    /// Current version
    current_version: Version,
    
    /// Author public key (for verification)
    author_pubkey: PublicKey,
    
    /// Master server URL
    master_url: String,
    
    /// P2P node for chunk distribution
    p2p_node: Arc<P2PNode>,
    
    /// Update state
    state: Arc<RwLock<UpdateState>>,
}

#[derive(Debug, Clone)]
pub enum UpdateState {
    Idle,
    CheckingForUpdate,
    UpdateAvailable(UpdateManifest),
    Downloading {
        manifest: UpdateManifest,
        progress: f32,
    },
    Verifying(UpdateManifest),
    Applying(UpdateManifest),
    Applied(Version),
    Failed {
        version: Version,
        error: String,
    },
    RollingBack {
        from_version: Version,
        to_version: Version,
    },
}

impl UpdateClient {
    pub fn new(
        current_version: Version,
        author_pubkey: PublicKey,
        master_url: String,
        p2p_node: Arc<P2PNode>,
    ) -> Self {
        Self {
            current_version,
            author_pubkey,
            master_url,
            p2p_node,
            state: Arc::new(RwLock::new(UpdateState::Idle)),
        }
    }
    
    /// Check for updates
    pub async fn check_for_update(&self) -> Result<Option<UpdateManifest>, UpdateError> {
        *self.state.write().await = UpdateState::CheckingForUpdate;
        
        // Query master server for latest version
        let url = format!("{}/api/v1/updates/latest", self.master_url);
        
        let response = reqwest::get(&url)
            .await
            .map_err(|e| UpdateError::NetworkError(e.to_string()))?;
        
        if response.status() == 404 {
            // No update available
            *self.state.write().await = UpdateState::Idle;
            return Ok(None);
        }
        
        let manifest: UpdateManifest = response
            .json()
            .await
            .map_err(|e| UpdateError::ParseError(e.to_string()))?;
        
        // Verify signature
        if !manifest.verify(&self.author_pubkey)? {
            return Err(UpdateError::SignatureInvalid(
                "Manifest signature verification failed".to_string()
            ));
        }
        
        // Check if update is applicable
        if manifest.version <= self.current_version {
            *self.state.write().await = UpdateState::Idle;
            return Ok(None);
        }
        
        if self.current_version < manifest.min_version {
            return Err(UpdateError::VersionTooOld(
                format!(
                    "Current version {} is too old. Minimum version required: {}",
                    self.current_version, manifest.min_version
                )
            ));
        }
        
        *self.state.write().await = UpdateState::UpdateAvailable(manifest.clone());
        
        Ok(Some(manifest))
    }
    
    /// Download update via P2P
    pub async fn download_update(
        &self,
        manifest: &UpdateManifest,
    ) -> Result<(), UpdateError> {
        *self.state.write().await = UpdateState::Downloading {
            manifest: manifest.clone(),
            progress: 0.0,
        };
        
        let temp_dir = std::env::temp_dir().join(format!("scm-update-{}", manifest.version));
        std::fs::create_dir_all(&temp_dir)
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        // Download each file via P2P
        let total_files = manifest.files.len();
        
        for (idx, file) in manifest.files.iter().enumerate() {
            // Request file from P2P mesh
            let content = self.p2p_node.fetch_content(&file.cid).await
                .map_err(|e| UpdateError::DownloadFailed(e.to_string()))?;
            
            // Verify hash
            let actual_hash = Self::hash_content(&content);
            if actual_hash != file.hash {
                return Err(UpdateError::HashMismatch {
                    expected: file.hash.clone(),
                    actual: actual_hash,
                });
            }
            
            // Write to temp directory
            let file_path = temp_dir.join(&file.path);
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| UpdateError::IoError(e.to_string()))?;
            }
            
            std::fs::write(&file_path, &content)
                .map_err(|e| UpdateError::IoError(e.to_string()))?;
            
            // Update progress
            let progress = (idx + 1) as f32 / total_files as f32;
            *self.state.write().await = UpdateState::Downloading {
                manifest: manifest.clone(),
                progress,
            };
        }
        
        Ok(())
    }
    
    /// Apply update
    pub async fn apply_update(
        &self,
        manifest: &UpdateManifest,
    ) -> Result<(), UpdateError> {
        *self.state.write().await = UpdateState::Verifying(manifest.clone());
        
        // Create backup
        let backup_dir = self.create_backup().await?;
        
        *self.state.write().await = UpdateState::Applying(manifest.clone());
        
        let temp_dir = std::env::temp_dir().join(format!("scm-update-{}", manifest.version));
        let install_dir = std::env::current_exe()
            .map_err(|e| UpdateError::IoError(e.to_string()))?
            .parent()
            .ok_or_else(|| UpdateError::IoError("Cannot determine install directory".to_string()))?
            .to_path_buf();
        
        // Apply each file
        for file in &manifest.files {
            let temp_path = temp_dir.join(&file.path);
            let target_path = install_dir.join(&file.path);
            
            if file.delta {
                // Apply delta patch
                let base_path = &target_path;
                let delta_path = &temp_path;
                let output_path = target_path.with_extension("new");
                
                self.apply_delta_patch(base_path, delta_path, &output_path).await?;
                
                // Replace original
                std::fs::rename(&output_path, &target_path)
                    .map_err(|e| UpdateError::IoError(e.to_string()))?;
            } else {
                // Direct replacement
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| UpdateError::IoError(e.to_string()))?;
                }
                
                std::fs::copy(&temp_path, &target_path)
                    .map_err(|e| UpdateError::IoError(e.to_string()))?;
            }
        }
        
        // Write update marker
        let marker_path = install_dir.join(".update_applied");
        std::fs::write(
            marker_path,
            format!("{}\n{}", manifest.version, Utc::now().to_rfc3339())
        )
        .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        // Cleanup temp directory
        let _ = std::fs::remove_dir_all(temp_dir);
        
        *self.state.write().await = UpdateState::Applied(manifest.version.clone());
        
        Ok(())
    }
    
    /// Create backup of current installation
    async fn create_backup(&self) -> Result<PathBuf, UpdateError> {
        let install_dir = std::env::current_exe()
            .map_err(|e| UpdateError::IoError(e.to_string()))?
            .parent()
            .ok_or_else(|| UpdateError::IoError("Cannot determine install directory".to_string()))?
            .to_path_buf();
        
        let backup_dir = install_dir.join("backups").join(self.current_version.to_string());
        
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        // Copy critical files
        let critical_files = vec![
            "sc-manager.exe",
            "config.toml",
            "data/",
        ];
        
        for file in critical_files {
            let src = install_dir.join(file);
            let dst = backup_dir.join(file);
            
            if src.exists() {
                if src.is_dir() {
                    Self::copy_dir_recursive(&src, &dst)?;
                } else {
                    std::fs::copy(&src, &dst)
                        .map_err(|e| UpdateError::IoError(e.to_string()))?;
                }
            }
        }
        
        Ok(backup_dir)
    }
    
    /// Rollback to previous version
    pub async fn rollback(&self, to_version: Version) -> Result<(), UpdateError> {
        *self.state.write().await = UpdateState::RollingBack {
            from_version: self.current_version.clone(),
            to_version: to_version.clone(),
        };
        
        let install_dir = std::env::current_exe()
            .map_err(|e| UpdateError::IoError(e.to_string()))?
            .parent()
            .ok_or_else(|| UpdateError::IoError("Cannot determine install directory".to_string()))?
            .to_path_buf();
        
        let backup_dir = install_dir.join("backups").join(to_version.to_string());
        
        if !backup_dir.exists() {
            return Err(UpdateError::RollbackFailed(
                format!("Backup for version {} not found", to_version)
            ));
        }
        
        // Restore from backup
        Self::copy_dir_recursive(&backup_dir, &install_dir)?;
        
        // Write rollback marker
        let marker_path = install_dir.join(".rollback_applied");
        std::fs::write(
            marker_path,
            format!("{} -> {}\n{}", self.current_version, to_version, Utc::now().to_rfc3339())
        )
        .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        *self.state.write().await = UpdateState::Applied(to_version);
        
        Ok(())
    }
    
    /// Apply delta patch to file
    async fn apply_delta_patch(
        &self,
        base_file: &Path,
        delta_file: &Path,
        output_file: &Path,
    ) -> Result<(), UpdateError> {
        use std::io::{BufReader, BufWriter, Read, Write, Seek, SeekFrom};
        
        let base = File::open(base_file)
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        let delta = File::open(delta_file)
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        let output = File::create(output_file)
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        let mut base_reader = BufReader::with_capacity(1024 * 1024, base);
        let mut delta_reader = BufReader::with_capacity(1024 * 1024, delta);
        let mut output_writer = BufWriter::with_capacity(1024 * 1024, output);
        
        let mut buffer = vec![0u8; 1024 * 1024]; // 1MB chunks
        
        loop {
            // Read delta instruction
            let instruction = match self.read_delta_instruction(&mut delta_reader) {
                Ok(inst) => inst,
                Err(DeltaError::EndOfFile) => break,
                Err(e) => return Err(UpdateError::DeltaPatchFailed(e.to_string())),
            };
            
            match instruction {
                DeltaInstruction::Copy { offset, length } => {
                    // Copy from base file
                    base_reader.seek(SeekFrom::Start(offset))
                        .map_err(|e| UpdateError::IoError(e.to_string()))?;
                    
                    let mut remaining = length;
                    while remaining > 0 {
                        let to_read = remaining.min(buffer.len() as u64) as usize;
                        base_reader.read_exact(&mut buffer[..to_read])
                            .map_err(|e| UpdateError::IoError(e.to_string()))?;
                        output_writer.write_all(&buffer[..to_read])
                            .map_err(|e| UpdateError::IoError(e.to_string()))?;
                        remaining -= to_read as u64;
                    }
                }
                DeltaInstruction::Insert { data } => {
                    // Insert new data
                    output_writer.write_all(&data)
                        .map_err(|e| UpdateError::IoError(e.to_string()))?;
                }
            }
        }
        
        output_writer.flush()
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        Ok(())
    }
    
    fn read_delta_instruction(
        &self,
        reader: &mut BufReader<File>,
    ) -> Result<DeltaInstruction, DeltaError> {
        use std::io::Read;
        
        let mut opcode = [0u8; 1];
        if reader.read_exact(&mut opcode).is_err() {
            return Err(DeltaError::EndOfFile);
        }
        
        match opcode[0] {
            0 => {
                // Copy instruction
                let mut offset_bytes = [0u8; 8];
                let mut length_bytes = [0u8; 8];
                reader.read_exact(&mut offset_bytes)?;
                reader.read_exact(&mut length_bytes)?;
                
                let offset = u64::from_le_bytes(offset_bytes);
                let length = u64::from_le_bytes(length_bytes);
                
                Ok(DeltaInstruction::Copy { offset, length })
            }
            1 => {
                // Insert instruction
                let mut length_bytes = [0u8; 4];
                reader.read_exact(&mut length_bytes)?;
                let length = u32::from_le_bytes(length_bytes) as usize;
                
                let mut data = vec![0u8; length];
                reader.read_exact(&mut data)?;
                
                Ok(DeltaInstruction::Insert { data })
            }
            _ => Err(DeltaError::InvalidOpcode),
        }
    }
    
    fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), UpdateError> {
        std::fs::create_dir_all(dst)
            .map_err(|e| UpdateError::IoError(e.to_string()))?;
        
        for entry in std::fs::read_dir(src)
            .map_err(|e| UpdateError::IoError(e.to_string()))?
        {
            let entry = entry.map_err(|e| UpdateError::IoError(e.to_string()))?;
            let file_type = entry.file_type()
                .map_err(|e| UpdateError::IoError(e.to_string()))?;
            
            let dst_path = dst.join(entry.file_name());
            
            if file_type.is_dir() {
                Self::copy_dir_recursive(&entry.path(), &dst_path)?;
            } else {
                std::fs::copy(&entry.path(), &dst_path)
                    .map_err(|e| UpdateError::IoError(e.to_string()))?;
            }
        }
        
        Ok(())
    }
    
    fn hash_content(content: &[u8]) -> String {
        use sha3::{Digest, Sha3_256};
        let mut hasher = Sha3_256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
enum DeltaInstruction {
    Copy { offset: u64, length: u64 },
    Insert { data: Vec<u8> },
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// ERROR TYPES
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Signature invalid: {0}")]
    SignatureInvalid(String),
    
    #[error("Version too old: {0}")]
    VersionTooOld(String),
    
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    
    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Delta patch failed: {0}")]
    DeltaPatchFailed(String),
    
    #[error("Rollback failed: {0}")]
    RollbackFailed(String),
}

#[derive(Debug, thiserror::Error)]
enum DeltaError {
    #[error("End of file")]
    EndOfFile,
    
    #[error("Invalid opcode")]
    InvalidOpcode,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// TESTS
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_update_manifest_creation() {
        let files = vec![
            FileEntry {
                path: "sc-manager.exe".to_string(),
                hash: "abc123".to_string(),
                size: 10_000_000,
                cid: "QmTest123".to_string(),
                delta: true,
                base_version: Some(Version::new(7, 1, 1)),
            },
        ];
        
        let manifest = UpdateManifest::new(
            Version::new(8, 0, 0),
            ReleaseChannel::Alpha,
            UpdateType::Major,
            Version::new(7, 0, 0),
            files,
            "https://scmanager.io/changelog/v8.0.0".to_string(),
            true,
        );
        
        assert_eq!(manifest.version, Version::new(8, 0, 0));
        assert_eq!(manifest.total_size, 10_000_000);
        assert!(manifest.required);
    }
    
    #[test]
    fn test_signature_verification() {
        use ed25519_dalek::Keypair;
        use rand::rngs::OsRng;
        
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        
        let mut manifest = UpdateManifest::new(
            Version::new(8, 0, 0),
            ReleaseChannel::Alpha,
            UpdateType::Major,
            Version::new(7, 0, 0),
            vec![],
            "https://scmanager.io/changelog/v8.0.0".to_string(),
            true,
        );
        
        manifest.sign(&keypair).unwrap();
        
        assert!(manifest.verify(&keypair.public).unwrap());
    }
    
    #[tokio::test]
    async fn test_update_state_transitions() {
        let mut state = UpdateState::Idle;
        
        // Check for update
        state = UpdateState::CheckingForUpdate;
        assert!(matches!(state, UpdateState::CheckingForUpdate));
        
        // Update available
        let manifest = UpdateManifest::new(
            Version::new(8, 0, 0),
            ReleaseChannel::Alpha,
            UpdateType::Major,
            Version::new(7, 0, 0),
            vec![],
            "https://scmanager.io/changelog/v8.0.0".to_string(),
            true,
        );
        
        state = UpdateState::UpdateAvailable(manifest.clone());
        assert!(matches!(state, UpdateState::UpdateAvailable(_)));
        
        // Downloading
        state = UpdateState::Downloading {
            manifest: manifest.clone(),
            progress: 0.5,
        };
        assert!(matches!(state, UpdateState::Downloading { progress, .. } if progress == 0.5));
    }
}
```

---

## 🧱 PART 2: ABSOLUTE CODE GENERATION RULES

### 2.1 Language & Runtime

```yaml
Rust:
  Version: Edition 2024 (Latest Stable)
  Compiler: rustc 1.75+
  
  Mandatory_Crates:
    - tokio = "1.35" (async runtime)
    - serde = "1.0" (serialization)
    - thiserror = "1.0" (errors)
    - anyhow = "1.0" (error context)
    - tracing = "0.1" (logging)
    - chrono = "0.4" (dates/times)
  
  Forbidden:
    - unwrap() in production code
    - expect() in production code
    - panic!() in production code
    - unsafe (unless absolutely necessary + documented)
    - .clone() without justification
    - Arc<Mutex<T>> (prefer RwLock or channels)

TypeScript:
  Version: 5.0+
  Mode: strict
  Runtime: Bun (preferred) or Node 20+
  
  Mandatory:
    - Biome (linting + formatting)
    - No 'any' types
    - Explicit return types
    - Error handling (Result pattern or throw)
  
  Forbidden:
    - var (use const/let)
    - == (use ===)
    - Function declarations (use arrow functions)
    - Callbacks (use async/await)
```

### 2.2 Architecture Rules

```yaml
Layer_Separation:
  Domain:
    - Business logic ONLY
    - Pure functions (no side effects)
    - No I/O, no async (unless domain requires)
    - No dependencies on other layers
    - Rich domain models (not anemic)
  
  Application:
    - Use cases + orchestration
    - Calls domain + repositories
    - Transaction boundaries
    - Error handling + mapping
  
  Adapter:
    - External system integration
    - Implements repository traits
    - Data transformation (DTO ↔ Domain)
    - Connection management
  
  Infrastructure:
    - Technical concerns (DB, logging, etc.)
    - Configuration
    - Dependency injection
    - Main entry point

Dependency_Rule:
  - Domain depends on: NOTHING
  - Application depends on: Domain
  - Adapter depends on: Domain + Application
  - Infrastructure depends on: ALL
  
  Direction: Always INWARD
  
  Violation_Example:
    ❌ Domain imports reqwest
    ❌ Domain imports tokio::fs
    ❌ Application imports RepositoryImpl
  
  Correct_Example:
    ✅ Domain defines trait Repository
    ✅ Adapter implements Repository
    ✅ Application uses trait Repository
```

### 2.3 Error Handling

```yaml
Pattern: Result<T, E>

Error_Types:
  - Use thiserror for domain errors
  - Use anyhow for application errors
  - Always include context
  - Chain errors (don't swallow)

Example:
  ```rust
  // Domain error (thiserror)
  #[derive(Debug, thiserror::Error)]
  pub enum OrganizationError {
      #[error("Organization not found: {id}")]
      NotFound { id: String },
      
      #[error("Member already exists: {handle}")]
      MemberExists { handle: String },
  }
  
  // Application error (anyhow with context)
  pub async fn create_organization(
      name: String,
  ) -> anyhow::Result<Organization> {
      let org = Organization::new(name)
          .context("Failed to create organization")?;
      
      repo.save(&org)
          .await
          .context("Failed to save organization to database")?;
      
      Ok(org)
  }
  ```

Forbidden:
  ❌ unwrap()
  ❌ expect()
  ❌ panic!()
  ❌ .ok() (swallows error)
  ❌ let _ = (ignores error)

Allowed_Exceptions:
  ✅ Tests can use .unwrap()
  ✅ Impossible states can use .expect("Impossible")
  ✅ Must document why it's impossible
```

### 2.4 Performance Rules

```yaml
Hot_Paths:
  - Profile before optimizing
  - Zero-copy where possible
  - Avoid allocations in loops
  - Use iterators (not collect unnecessarily)
  - Prefer &str over String
  - Prefer &[T] over Vec<T>

Async:
  - Use tokio for I/O-bound
  - Use rayon for CPU-bound
  - Never block async runtime
  - Use spawn_blocking for blocking calls

Memory:
  - No memory leaks (use Arc wisely)
  - Drop large objects ASAP
  - Use streaming for large files
  - Memory-map large read-only files

Caching:
  - Cache expensive operations
  - Invalidate caches proactively
  - Use TTL for time-sensitive data
  - LRU for size-limited caches
```

---

## 🧪 PART 3: TESTING RULES

### 3.1 Test Coverage

```yaml
Minimum_Coverage: 85%

Unit_Tests:
  - EVERY public function
  - EVERY error path
  - EVERY edge case
  - Fast (<1ms per test)

Integration_Tests:
  - EVERY adapter
  - EVERY repository
  - EVERY external API
  - Database transactions
  - Can be slower (<100ms per test)

E2E_Tests:
  - EVERY user flow
  - Happy paths + error paths
  - Can be slow (<1s per test)

Performance_Tests:
  - Critical paths only
  - Latency benchmarks (p50, p95, p99)
  - Throughput benchmarks
  - Memory usage
```

### 3.2 Test Structure

```yaml
Pattern: Arrange-Act-Assert

Example:
  ```rust
  #[tokio::test]
  async fn test_create_organization_success() {
      // Arrange
      let repo = MockOrganizationRepository::new();
      let service = OrganizationService::new(repo);
      let name = "Test Org".to_string();
      
      // Act
      let result = service.create(name.clone()).await;
      
      // Assert
      assert!(result.is_ok());
      let org = result.unwrap();
      assert_eq!(org.name, name);
      assert!(org.id.len() > 0);
  }
  
  #[tokio::test]
  async fn test_create_organization_duplicate_name() {
      // Arrange
      let mut repo = MockOrganizationRepository::new();
      repo.add_existing("Test Org");
      let service = OrganizationService::new(repo);
      
      // Act
      let result = service.create("Test Org".to_string()).await;
      
      // Assert
      assert!(result.is_err());
      let err = result.unwrap_err();
      assert!(err.to_string().contains("already exists"));
  }
  ```

Naming:
  - test_{function}_{scenario}
  - Example: test_create_organization_success
  - Example: test_update_member_not_found
```

---

## 🔐 PART 4: SECURITY RULES

### 4.1 CIG ToS Compliance (HIGHEST PRIORITY)

```yaml
Prohibited_Actions:
  NEVER:
    - Inject into game process
    - Modify game memory
    - Intercept network traffic
    - Tamper with game files
    - Automate gameplay
    - Provide unfair advantage
    - Enable RMT (real money trading)

Allowed_Actions:
  ONLY:
    - Read Game.log (memory-mapped, read-only)
    - Call RSI API (OAuth 2.0)
    - Organize community (players)
    - Track statistics (manual verification)
    - Coordinate fleets (player-driven)

Verification:
  - ALL mission completions require officer approval
  - NO automatic UEC/reputation awards
  - NO real-time tactical advantages
  - Source tracking (Log/Manual/Officer)

Code_Check:
  BEFORE writing ANY code that touches:
    - Game files
    - RSI API
    - Mission tracking
    - Reputation system
  
  ASK:
    1. Does this manipulate the game? (NO = OK)
    2. Does this automate gameplay? (NO = OK)
    3. Does this provide unfair advantage? (NO = OK)
    4. Does this require manual verification? (YES = OK)
  
  IF ANY answer wrong → DO NOT IMPLEMENT
```

### 4.2 Authentication & Authorization

```yaml
RSI_OAuth:
  - PKCE flow (public client)
  - Tokens encrypted at rest (AES-256-GCM)
  - Automatic refresh
  - Revocation on logout

Local_Authentication:
  - Ed25519 keypair (hardware-bound optional)
  - Challenge-response
  - No password storage (biometric preferred)

Authorization:
  - RBAC (Role-Based Access Control)
  - Least privilege principle
  - Permission inheritance
  - Audit logging (all access)
```

### 4.3 Data Protection

```yaml
Encryption:
  At_Rest:
    - AES-256-GCM
    - Unique key per user
    - Key derivation (Argon2)
  
  In_Transit:
    - TLS 1.3
    - mTLS for P2P
    - Certificate pinning (author master)

Privacy:
  - NO PII off-device
  - Hash all identifiers (SHA3-256 + salt)
  - Local-first storage
  - Zero-knowledge architecture
  - Right to erasure (GDPR)

Sensitive_Data:
  Must_Encrypt:
    - RSI tokens
    - User credentials
    - Organization secrets
    - Diplomatic agreements
    - Private messages
  
  Never_Store:
    - Passwords (use tokens)
    - Credit cards (no payments)
    - Social Security Numbers
    - Government IDs
```

---

## 📝 PART 5: DOCUMENTATION RULES

### 5.1 Code Documentation

```yaml
Rust:
  - Every public item has doc comment
  - Explain WHY, not WHAT
  - Include examples for complex functions
  - Link to related functions
  - Note panics, errors, safety

  Example:
    ```rust
    /// Create a new organization
    ///
    /// This validates the organization name and generates a unique ID.
    /// The organization is NOT saved to the database - call `save()` separately.
    ///
    /// # Arguments
    /// * `name` - Organization name (3-50 characters)
    ///
    /// # Returns
    /// * `Ok(Organization)` - Created organization
    /// * `Err(OrganizationError::InvalidName)` - Name validation failed
    ///
    /// # Examples
    /// ```
    /// let org = Organization::new("Test Org".to_string())?;
    /// assert_eq!(org.name, "Test Org");
    /// ```
    pub fn new(name: String) -> Result<Self, OrganizationError> {
        // Implementation
    }
    ```

TypeScript:
  - JSDoc for all exported functions
  - Type annotations (explicit)
  - Examples for complex types

  Example:
    ```typescript
    /**
     * Create a new organization
     * 
     * @param name Organization name (3-50 characters)
     * @returns Created organization
     * @throws {ValidationError} If name is invalid
     * 
     * @example
     * const org = await createOrganization("Test Org");
     * console.log(org.id);
     */
    export async function createOrganization(
      name: string
    ): Promise<Organization> {
      // Implementation
    }
    ```
```

### 5.2 Architecture Documentation

```yaml
ADRs (Architecture Decision Records):
  - Create ADR for significant decisions
  - Format: Markdown in docs/adr/
  - Include: Context, Decision, Consequences
  - Number sequentially (0001, 0002, etc.)

README:
  - Every crate/package has README.md
  - Include: Purpose, Usage, Examples
  - Link to related documentation

Diagrams:
  - Use Mermaid for diagrams
  - Embed in Markdown
  - Keep up-to-date
```

---

## 🚨 PART 6: DECISION PROTOCOL

### 6.1 When Uncertain

```yaml
IF feature_not_specified:
  1. Check this specification
  2. Check architecture rules
  3. Use most conservative approach
  4. Generate code immediately
  5. Let CI/tests validate

IF core_or_plugin:
  - Core concern? → CORE
  - Optional? → PLUGIN
  - Still unsure? → PLUGIN (default)

IF tech_choice_unclear:
  - Use specified defaults
  - Never introduce new deps without spec
  - Stick to: tokio, serde, thiserror, anyhow

IF ToS_concern:
  - CIG ToS wins ALWAYS
  - If potentially violates → DO NOT IMPLEMENT
  - If unsure → DO NOT IMPLEMENT
  - Document reasoning
```

### 6.2 Code Generation Priority

```yaml
Order:
  1. Update system (THIS DOCUMENT)
  2. Domain models
  3. Application services
  4. Adapters
  5. Infrastructure
  6. UI
  7. Tests (alongside, not after)
  8. Documentation (alongside, not after)

Never:
  - Generate snippets (always full files)
  - Skip error handling
  - Skip tests
  - Skip documentation
  - Ask user questions (generate, let CI validate)
```

---

## ✅ PART 7: SELF-CHECK PROTOCOL

### Before Writing Any Code

```yaml
Checklist:
  - [ ] Specification section read?
  - [ ] Layer correct (Domain/Application/Adapter)?
  - [ ] Architecture rules followed?
  - [ ] Tech stack matches spec?
  - [ ] Error handling complete (Result<T, E>)?
  - [ ] Tests included?
  - [ ] Documentation included?
  - [ ] Performance considered?
  - [ ] Security rules followed?
  - [ ] ToS compliance verified?
  - [ ] CIG ToS specifically checked?
  - [ ] IDC-10 guidelines met (if Windows)?
  - [ ] No unwrap/expect/panic?

IF ANY checkbox unchecked:
  - STOP
  - Review specification
  - Fix issue
  - THEN generate code
```

---

## 🎯 PART 8: FILE STRUCTURE (ABSOLUTE)

```yaml
workspace/
  .github/
    workflows/
      ci.yml              # CI pipeline
      release.yml         # Release pipeline
  
  apps/
    desktop/              # Tauri desktop app
      src-tauri/          # Rust backend
      src/                # SolidJS frontend
    master-server/        # Author Master Server
  
  core/
    domain/               # Business logic
      organizations/
      members/
      operations/
      fleet/
      diplomacy/
    application/          # Use cases
      commands/
      queries/
    events/               # Domain events
  
  infrastructure/
    persistence/
      rocksdb/
      postgresql/
    eventbus/
    p2p-mesh/
      crdt/
      gossip/
    update-system/        # ← FIRST TO IMPLEMENT
      author/
      client/
      delta/
      signature/
      rollback/
    master-server/
    plugin-sdk/
    installer/
  
  adapters/
    adapter-rsi-auth/
    adapter-discord/
    adapter-game-log/
    # ... other adapters
  
  plugins/
    grinding/
    roleplay/
    # ... other plugins
  
  docs/
    api/                  # API documentation
    architecture/         # Architecture docs
      adr/                # Decision records
    guides/               # User/dev guides
  
  tests/
    unit/
    integration/
    e2e/
    performance/
  
  scripts/
    build.sh
    test.sh
    deploy.sh
  
  Cargo.toml              # Workspace root
  package.json            # Frontend workspace
  .gitignore
  README.md
```

---

## 🔄 PART 9: CI/CD PIPELINE

```yaml
On_Push:
  - Format check (rustfmt, biome)
  - Lint (clippy --deny warnings, biome)
  - Build (all targets: Windows, Linux, macOS)
  - Test (unit + integration)
  - Coverage (>85% required)
  - Security audit (cargo audit)
  - License check (cargo deny)

On_PR:
  - All of above
  - E2E tests
  - Performance benchmarks
  - Mutation testing (cargo-mutants)

On_Release_Tag:
  - All of above
  - Build release binaries (all platforms)
  - Sign binaries (Ed25519)
  - Generate checksums (SHA3-512)
  - Create update manifest
  - Publish to Master Server
  - Deploy documentation
  - Create GitHub release

Local_CI:
  - Use 'act' for GitHub Actions locally
  - Fast feedback loop (<5 min)
  - Same pipeline as CI
```

---

## 📊 PART 10: METRICS & MONITORING

```yaml
What_To_Log:
  ERROR:
    - Exceptions
    - Failed operations
    - Security violations
  
  WARN:
    - Retries
    - Degraded performance
    - Deprecated feature usage
  
  INFO:
    - User actions (high-level)
    - System state changes
    - Background task completion
  
  DEBUG:
    - Detailed execution flow
    - Variable values
    - Timing information
  
  TRACE:
    - Everything (development only)

What_NOT_To_Log:
  - Passwords/tokens (NEVER)
  - PII (without explicit consent)
  - Full request/response bodies
  - Sensitive data

Structured_Logging:
  ```rust
  use tracing::{info, error, instrument};
  
  #[instrument(skip(password))]
  pub async fn login(username: &str, password: &str) -> Result<()> {
      info!(username = %username, "User attempting login");
      
      match authenticate(username, password).await {
          Ok(token) => {
              info!(username = %username, "Login successful");
              Ok(())
          }
          Err(e) => {
              error!(
                  username = %username,
                  error = %e,
                  "Login failed"
              );
              Err(e)
          }
      }
  }
  ```
```

---

## 🎓 PART 11: LEARNING & IMPROVEMENT

```yaml
After_Each_Implementation:
  - Review generated code
  - Check test coverage
  - Run benchmarks
  - Update documentation
  - Commit ADR if significant decision

After_Each_Sprint:
  - Review metrics
  - Identify bottlenecks
  - Update performance budgets
  - Refactor hot paths
  - Update this instruction (if needed)

Continuous:
  - Monitor Rust ecosystem (new crates, patterns)
  - Monitor security advisories
  - Monitor CIG ToS updates
  - Monitor community feedback
```

---

## ⚡ PART 12: IMMEDIATE ACTION ITEMS

```yaml
Task_001_Update_System: ← START HERE
  Priority: CRITICAL
  Effort: 3 days
  Files_To_Generate:
    - infrastructure/update-system/src/lib.rs (DONE ABOVE)
    - infrastructure/update-system/src/author/mod.rs
    - infrastructure/update-system/src/client/mod.rs
    - infrastructure/update-system/src/delta/mod.rs
    - infrastructure/update-system/src/signature/mod.rs
    - infrastructure/update-system/src/rollback/mod.rs
    - infrastructure/update-system/tests/integration_tests.rs
    - infrastructure/update-system/Cargo.toml
    - infrastructure/update-system/README.md
  
  Tests:
    - Unit: All functions
    - Integration: Full update flow
    - E2E: Real P2P update
    - Performance: Delta patch speed
  
  Acceptance_Criteria:
    - [ ] Update manifest creation works
    - [ ] Signature verification works
    - [ ] P2P download works
    - [ ] Delta patching works
    - [ ] Rollback works
    - [ ] All tests pass
    - [ ] Coverage >85%
    - [ ] Documentation complete

Task_002_P2P_Mesh:
  Priority: CRITICAL
  Effort: 5 days
  Dependencies: None (parallel with Task_001)
  
Task_003_Master_Server:
  Priority: CRITICAL
  Effort: 4 days
  Dependencies: Task_001 (needs signature module)

Task_004_Domain_Models:
  Priority: HIGH
  Effort: 3 days
  Dependencies: None
```

---

## 🏁 FINAL INSTRUCTIONS

```yaml
Execution_Mode: AUTONOMOUS
Questions_Allowed: ZERO
Deviations_Allowed: ZERO
Incomplete_Work: FORBIDDEN

When_I_Say_"Implement_X":
  1. Read this instruction completely
  2. Identify which part of architecture
  3. Generate COMPLETE files
  4. Include ALL imports
  5. Include ALL error handling
  6. Include ALL tests
  7. Include ALL documentation
  8. No snippets, no TODOs, no placeholders

Output_Format:
  - Full file path
  - Complete file content
  - Explanation (brief, technical)
  - Next recommended step

Remember:
  - CIG ToS is HIGHEST priority
  - Security before features
  - Tests before shipping
  - Documentation alongside code
  - No unwrap/expect/panic
  - Result<T, E> always
  - 85% coverage minimum

First_Task:
  Generate all files for infrastructure/update-system/
  Starting with the code above (lib.rs)
  Then author/, client/, delta/, signature/, rollback/
  Then tests/
  Then Cargo.toml, README.md
```

---

**COPILOT MASTER INSTRUCTION V8.0 - ABSOLUTE & BINDING**

**This instruction is COMPLETE, ABSOLUTE, and BINDING.**
**Zero questions. Zero deviations. Maximum execution efficiency.**

**First task: Implement complete update system as specified above.**

**Status: READY TO EXECUTE** ✅

