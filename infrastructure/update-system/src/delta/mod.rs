use sha3::{Digest, Sha3_256};
use thiserror::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Simple delta chunker used for splitting update payloads into fixed-size chunks.
/// Split `data` into chunks of at most `chunk_size` bytes. Last chunk may be smaller.
pub fn chunks(data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    if chunk_size == 0 {
        return vec![];
    }
    data.chunks(chunk_size).map(|c| c.to_vec()).collect()
}

#[derive(Debug, Error)]
pub enum DeltaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Hash mismatch")]
    HashMismatch,
}

/// Apply chunks atomically to `dest_path`. Writes to a temporary file in the same
/// directory then renames on success. Verifies SHA3-256 of assembled data matches
/// `expected_sha3_hex` before committing.
pub fn apply_chunks<P: AsRef<Path>>(chunks: &[Vec<u8>], dest_path: P, expected_sha3_hex: &str) -> Result<(), DeltaError> {
    let dest = dest_path.as_ref();
    let parent = dest.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;

    let tmp = dest.with_extension("tmp.partial");
    // Ensure the tmp file is removed on error
    let mut file = fs::File::create(&tmp)?;
    let mut hasher = Sha3_256::new();
    for c in chunks {
        file.write_all(c)?;
        hasher.update(c);
    }
    file.flush()?;

    let digest = hasher.finalize();
    let got = hex::encode(digest);
    if got != expected_sha3_hex {
        // remove partial file
        let _ = fs::remove_file(&tmp);
        return Err(DeltaError::HashMismatch);
    }

    // atomic rename
    fs::rename(&tmp, dest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Read;

    #[test]
    fn apply_chunks_success() {
        let dir = tempdir().unwrap();
        let dest = dir.path().join("out.bin");
        let data = vec![0u8; 3000];
        let chunks = chunks(&data, 1024);

        let mut hasher = Sha3_256::new();
        hasher.update(&data);
        let expected = hex::encode(hasher.finalize());

        apply_chunks(&chunks, &dest, &expected).unwrap();
        let mut buf = Vec::new();
        fs::File::open(&dest).unwrap().read_to_end(&mut buf).unwrap();
        assert_eq!(buf.len(), data.len());
        assert_eq!(buf, data);
    }

    #[test]
    fn apply_chunks_hash_mismatch() {
        let dir = tempdir().unwrap();
        let dest = dir.path().join("out.bin");
        let data = vec![1u8; 100];
        let chunks = chunks(&data, 32);

        let bad_expected = "deadbeef"; // intentionally wrong
        let res = apply_chunks(&chunks, &dest, bad_expected);
        assert!(matches!(res, Err(DeltaError::HashMismatch)));
        assert!(!dest.exists(), "dest should not exist on hash mismatch");
    }
}
