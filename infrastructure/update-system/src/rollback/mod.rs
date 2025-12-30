use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Error)]
pub enum RollbackError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Backup not found")]
    NotFound,
}

/// Create a simple backup by copying the file to `<orig>.bak` and returning the path.
pub fn create_backup(path: &Path) -> Result<PathBuf, RollbackError> {
    let dst = path.with_extension("bak");
    fs::copy(path, &dst)?;
    Ok(dst)
}

/// Atomically replace `dest` with `tmp`.
/// Steps:
/// 1. If `dest` exists, move it to `dest.key.json.bak.<ts>` (backup).
/// 2. Rename `tmp` to `dest` (atomic where supported).
/// 3. On failure, attempt to restore backup.
/// Returns `Ok(Some(backup_path))` if backup was created, `Ok(None)` if no prior `dest` existed.
pub fn transactional_replace(tmp: &Path, dest: &Path) -> Result<Option<PathBuf>, RollbackError> {
    let backup_path = if dest.exists() {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let bak = dest.with_extension(format!("bak.{}", ts));
        fs::rename(dest, &bak)?;
        Some(bak)
    } else {
        None
    };

    // Attempt rename tmp -> dest
    match fs::rename(tmp, dest) {
        Ok(()) => Ok(backup_path),
        Err(e) => {
            // Attempt to restore backup if present
            if let Some(ref bak) = backup_path {
                let _ = fs::rename(bak, dest);
            }
            Err(RollbackError::Io(e))
        }
    }
}

/// Restore a backup file to the destination
pub fn restore_backup(backup: &Path, dest: &Path) -> Result<(), RollbackError> {
    if !backup.exists() {
        return Err(RollbackError::NotFound);
    }
    fs::rename(backup, dest)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;

    #[test]
    fn transactional_replace_success() {
        let dir = tempdir().unwrap();
        let dest = dir.path().join("file.txt");
        let mut f = fs::File::create(&dest).unwrap();
        writeln!(f, "old").unwrap();

        let tmp = dir.path().join("file.tmp");
        let mut t = fs::File::create(&tmp).unwrap();
        writeln!(t, "new").unwrap();

        let bak = transactional_replace(&tmp, &dest).unwrap();
        assert!(bak.is_some());
        let bak_path = bak.unwrap();
        assert!(bak_path.exists());

        let content = fs::read_to_string(&dest).unwrap();
        assert!(content.contains("new"));

        let old = fs::read_to_string(&bak_path).unwrap();
        assert!(old.contains("old"));
    }

    #[test]
    fn transactional_replace_failure_restores() {
        let dir = tempdir().unwrap();
        let dest = dir.path().join("file.txt");
        let mut f = fs::File::create(&dest).unwrap();
        writeln!(f, "original").unwrap();

        let tmp = dir.path().join("file_missing.tmp");
        // Intentionally do NOT create tmp so rename will fail

        let res = transactional_replace(&tmp, &dest);
        assert!(res.is_err());

        // ensure original restored
        let content = fs::read_to_string(&dest).unwrap();
        assert!(content.contains("original"));

        // backup should have been restored; ensure destination still contains original content
        assert!(dest.exists());
        let content = fs::read_to_string(&dest).unwrap();
        assert!(content.contains("original"));
    }
}
