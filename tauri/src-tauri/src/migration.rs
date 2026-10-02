use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub const HODUSON_DATA_MIGRATION_VERSION: u32 = 1;
pub const OLD_BUNDLE_IDENTIFIER: &str = "sh.voicebox.app";
pub const NEW_BUNDLE_IDENTIFIER: &str = "io.github.hoduson.voicestudio";
pub const MIGRATION_MARKER_FILENAME: &str = "migration_marker.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MigrationMarker {
    pub migration_version: u32,
    pub source_identifier: String,
    pub target_identifier: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub status: String,
    pub files_copied: usize,
    pub total_bytes: u64,
    pub db_sha256: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MigrationStatus {
    NoSource,
    AlreadyCompleted,
    Migrated {
        files_copied: usize,
        total_bytes: u64,
        db_sha256: Option<String>,
    },
}

#[derive(Debug)]
pub enum MigrationError {
    Io(std::io::Error),
    Json(serde_json::Error),
    DestinationConflict(PathBuf),
    HashMismatch {
        file: PathBuf,
        expected: String,
        actual: String,
    },
    SourceReadError(String),
    SourceDatabaseActive(String),
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::Io(e) => write!(f, "IO error during migration: {}", e),
            MigrationError::Json(e) => write!(f, "JSON error in migration marker: {}", e),
            MigrationError::DestinationConflict(p) => {
                write!(f, "Destination conflict: file exists with different content at {:?}", p)
            }
            MigrationError::HashMismatch {
                file,
                expected,
                actual,
            } => write!(
                f,
                "Hash mismatch for {:?}: expected {}, got {}",
                file, expected, actual
            ),
            MigrationError::SourceReadError(msg) => write!(f, "Source read error: {}", msg),
            MigrationError::SourceDatabaseActive(msg) => write!(
                f,
                "Legacy Voicebox SQLite database is active ({}). Please close Voicebox completely, wait for backend process to terminate, and retry.",
                msg
            ),
        }
    }
}

impl std::error::Error for MigrationError {}

impl From<std::io::Error> for MigrationError {
    fn from(e: std::io::Error) -> Self {
        MigrationError::Io(e)
    }
}

impl From<serde_json::Error> for MigrationError {
    fn from(e: serde_json::Error) -> Self {
        MigrationError::Json(e)
    }
}

pub fn compute_sha256(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = std::io::Read::read(&mut file, &mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn iso_now() -> String {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Convert seconds to approximate ISO8601 UTC
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let mins = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    // Unix epoch starts 1970-01-01
    let mut y = 1970i64;
    let mut rem_days = days as i64;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if rem_days >= days_in_year {
            rem_days -= days_in_year;
            y += 1;
        } else {
            break;
        }
    }

    let leap = (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0);
    let days_in_month = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut m = 1;
    for &dim in &days_in_month {
        if rem_days >= dim {
            rem_days -= dim;
            m += 1;
        } else {
            break;
        }
    }
    let d = rem_days + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hours, mins, seconds
    )
}

fn is_excluded(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower == ".keep-running"
        || lower.ends_with(".lock")
        || lower.ends_with(".pid")
        || lower.ends_with(".tmp")
        || lower.ends_with(".db-wal")
        || lower.ends_with(".db-shm")
        || lower == "cache"
        || lower == "ebwebview"
        || lower == "logs"
        || lower == MIGRATION_MARKER_FILENAME
        || lower.starts_with(".migration")
}

fn copy_file_safe(src: &Path, dst: &Path) -> Result<(bool, u64), MigrationError> {
    let src_meta = src
        .metadata()
        .map_err(|e| MigrationError::SourceReadError(format!("Cannot read {:?}: {}", src, e)))?;
    let src_len = src_meta.len();

    if dst.exists() {
        let dst_len = dst.metadata()?.len();
        if src_len == dst_len {
            let src_hash = compute_sha256(src)?;
            let dst_hash = compute_sha256(dst)?;
            if src_hash == dst_hash {
                return Ok((false, src_len));
            }
        }
        return Err(MigrationError::DestinationConflict(dst.to_path_buf()));
    }

    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::copy(src, dst)?;

    let dst_len = dst.metadata()?.len();
    if dst_len != src_len {
        return Err(MigrationError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Size mismatch after copy of {:?}: expected {}, got {}",
                dst, src_len, dst_len
            ),
        )));
    }

    if src
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        == Some("voicebox.db".to_string())
    {
        let src_hash = compute_sha256(src)?;
        let dst_hash = compute_sha256(dst)?;
        if src_hash != dst_hash {
            return Err(MigrationError::HashMismatch {
                file: dst.to_path_buf(),
                expected: src_hash,
                actual: dst_hash,
            });
        }
    }

    Ok((true, src_len))
}

fn copy_dir_recursive(
    src_dir: &Path,
    dst_dir: &Path,
    files_copied: &mut usize,
    total_bytes: &mut u64,
) -> Result<(), MigrationError> {
    if !dst_dir.exists() {
        std::fs::create_dir_all(dst_dir)?;
    }

    let entries = std::fs::read_dir(src_dir)
        .map_err(|e| MigrationError::SourceReadError(format!("Failed to read dir {:?}: {}", src_dir, e)))?;

    for entry in entries {
        let entry = entry
            .map_err(|e| MigrationError::SourceReadError(format!("Failed to read entry in {:?}: {}", src_dir, e)))?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        if is_excluded(&name_str) {
            continue;
        }

        let child_src = entry.path();
        let child_dst = dst_dir.join(&file_name);

        if file_type.is_dir() {
            copy_dir_recursive(&child_src, &child_dst, files_copied, total_bytes)?;
        } else if file_type.is_file() {
            let (copied, bytes) = copy_file_safe(&child_src, &child_dst)?;
            if copied {
                *files_copied += 1;
            }
            *total_bytes += bytes;
        }
    }

    Ok(())
}

pub fn migrate_data_dir(
    source_root: &Path,
    target_root: &Path,
) -> Result<MigrationStatus, MigrationError> {
    if !source_root.exists() {
        return Ok(MigrationStatus::NoSource);
    }

    let marker_path = target_root.join(MIGRATION_MARKER_FILENAME);
    if marker_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&marker_path) {
            if let Ok(marker) = serde_json::from_str::<MigrationMarker>(&content) {
                if marker.status == "completed"
                    && marker.migration_version == HODUSON_DATA_MIGRATION_VERSION
                {
                    return Ok(MigrationStatus::AlreadyCompleted);
                }
            }
        }
    }

    // SQLite Source Quiescence Guard: check for active WAL/SHM in legacy source
    let wal_file = source_root.join("voicebox.db-wal");
    let shm_file = source_root.join("voicebox.db-shm");
    if wal_file.exists() || shm_file.exists() {
        let mut active = Vec::new();
        if wal_file.exists() {
            active.push("voicebox.db-wal");
        }
        if shm_file.exists() {
            active.push("voicebox.db-shm");
        }
        return Err(MigrationError::SourceDatabaseActive(active.join(", ")));
    }

    std::fs::create_dir_all(target_root)?;

    let started_at = iso_now();
    let initial_marker = MigrationMarker {
        migration_version: HODUSON_DATA_MIGRATION_VERSION,
        source_identifier: OLD_BUNDLE_IDENTIFIER.to_string(),
        target_identifier: NEW_BUNDLE_IDENTIFIER.to_string(),
        started_at: started_at.clone(),
        completed_at: None,
        status: "in_progress".to_string(),
        files_copied: 0,
        total_bytes: 0,
        db_sha256: None,
    };
    std::fs::write(&marker_path, serde_json::to_string_pretty(&initial_marker)?)?;

    let mut files_copied = 0usize;
    let mut total_bytes = 0u64;

    let entries = std::fs::read_dir(source_root)
        .map_err(|e| MigrationError::SourceReadError(format!("Failed to read source {:?}: {}", source_root, e)))?;

    for entry in entries {
        let entry = entry
            .map_err(|e| MigrationError::SourceReadError(format!("Failed entry in {:?}: {}", source_root, e)))?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();
        if is_excluded(&name_str) {
            continue;
        }

        let child_src = entry.path();
        let child_dst = target_root.join(&file_name);

        if file_type.is_dir() {
            copy_dir_recursive(&child_src, &child_dst, &mut files_copied, &mut total_bytes)?;
        } else if file_type.is_file() {
            let (copied, bytes) = copy_file_safe(&child_src, &child_dst)?;
            if copied {
                files_copied += 1;
            }
            total_bytes += bytes;
        }
    }

    let target_db = target_root.join("voicebox.db");
    let db_sha256 = if target_db.exists() {
        Some(compute_sha256(&target_db)?)
    } else {
        None
    };

    let completed_at = iso_now();
    let completed_marker = MigrationMarker {
        migration_version: HODUSON_DATA_MIGRATION_VERSION,
        source_identifier: OLD_BUNDLE_IDENTIFIER.to_string(),
        target_identifier: NEW_BUNDLE_IDENTIFIER.to_string(),
        started_at,
        completed_at: Some(completed_at),
        status: "completed".to_string(),
        files_copied,
        total_bytes,
        db_sha256: db_sha256.clone(),
    };
    std::fs::write(&marker_path, serde_json::to_string_pretty(&completed_marker)?)?;

    Ok(MigrationStatus::Migrated {
        files_copied,
        total_bytes,
        db_sha256,
    })
}

pub fn resolve_legacy_data_dir(target_data_dir: &Path) -> Option<PathBuf> {
    target_data_dir.parent().map(|p| p.join(OLD_BUNDLE_IDENTIFIER))
}

pub fn run_migration_if_needed(target_data_dir: &Path) -> Result<MigrationStatus, String> {
    let legacy_dir = match resolve_legacy_data_dir(target_data_dir) {
        Some(dir) => dir,
        None => return Ok(MigrationStatus::NoSource),
    };

    println!(
        "HODUSON Migration: Checking migration from {:?} to {:?}",
        legacy_dir, target_data_dir
    );
    match migrate_data_dir(&legacy_dir, target_data_dir) {
        Ok(status) => {
            match &status {
                MigrationStatus::NoSource => {
                    println!(
                        "HODUSON Migration: No legacy data directory found at {:?}. Clean install.",
                        legacy_dir
                    );
                }
                MigrationStatus::AlreadyCompleted => {
                    println!(
                        "HODUSON Migration: Already migrated to {:?}. Skipping.",
                        target_data_dir
                    );
                }
                MigrationStatus::Migrated {
                    files_copied,
                    total_bytes,
                    db_sha256,
                } => {
                    println!(
                        "HODUSON Migration: Successfully migrated {} files ({} bytes) to {:?}. DB SHA256: {:?}",
                        files_copied, total_bytes, target_data_dir, db_sha256
                    );
                }
            }
            Ok(status)
        }
        Err(e) => {
            eprintln!("HODUSON Migration ERROR: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_case_1_old_absent_new_absent() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("old_absent");
        let new_dir = base.path().join("new_absent");

        let res = migrate_data_dir(&old_dir, &new_dir).expect("should succeed");
        assert_eq!(res, MigrationStatus::NoSource);
        assert!(!new_dir.exists(), "target dir should not be created if no source exists");
    }

    #[test]
    fn test_case_2_old_exists_new_absent() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(old_dir.join("generations")).unwrap();
        std::fs::create_dir_all(old_dir.join("profiles").join("prof-1")).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"SQLite format 3\0test_db_bytes").unwrap();
        std::fs::write(old_dir.join("generations").join("gen1.wav"), b"WAV_DATA_1").unwrap();
        std::fs::write(old_dir.join("profiles").join("prof-1").join("sample.bin"), b"SAMPLE_DATA").unwrap();

        let old_db_hash = compute_sha256(&old_dir.join("voicebox.db")).unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir).expect("migration should succeed");
        match res {
            MigrationStatus::Migrated { files_copied, total_bytes, db_sha256 } => {
                assert_eq!(files_copied, 3);
                assert!(total_bytes > 0);
                assert_eq!(db_sha256, Some(old_db_hash.clone()));
            }
            other => panic!("Unexpected result: {:?}", other),
        }

        assert!(new_dir.join("voicebox.db").exists());
        assert_eq!(compute_sha256(&new_dir.join("voicebox.db")).unwrap(), old_db_hash);
        assert_eq!(std::fs::read(new_dir.join("generations").join("gen1.wav")).unwrap(), b"WAV_DATA_1");
        assert_eq!(std::fs::read(new_dir.join("profiles").join("prof-1").join("sample.bin")).unwrap(), b"SAMPLE_DATA");

        let marker: MigrationMarker = serde_json::from_str(&std::fs::read_to_string(new_dir.join(MIGRATION_MARKER_FILENAME)).unwrap()).unwrap();
        assert_eq!(marker.status, "completed");
        assert_eq!(marker.migration_version, HODUSON_DATA_MIGRATION_VERSION);
        assert_eq!(marker.source_identifier, OLD_BUNDLE_IDENTIFIER);
        assert_eq!(marker.target_identifier, NEW_BUNDLE_IDENTIFIER);

        // Verify source was NOT deleted or altered
        assert!(old_dir.join("voicebox.db").exists());
        assert_eq!(compute_sha256(&old_dir.join("voicebox.db")).unwrap(), old_db_hash);
    }

    #[test]
    fn test_case_3_old_exists_new_exists_but_empty() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::create_dir_all(&new_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"SQLite format 3\0content").unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir).expect("should succeed");
        assert!(matches!(res, MigrationStatus::Migrated { .. }));
        assert!(new_dir.join("voicebox.db").exists());
    }

    #[test]
    fn test_case_4_old_exists_new_contains_completed_marker() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"OLD_DATA").unwrap();

        // Run once
        let res1 = migrate_data_dir(&old_dir, &new_dir).unwrap();
        assert!(matches!(res1, MigrationStatus::Migrated { .. }));

        // Modify a new file in target to simulate user activity
        std::fs::write(new_dir.join("new_session.txt"), b"USER_NEW_WORK").unwrap();

        // Run second time
        let res2 = migrate_data_dir(&old_dir, &new_dir).unwrap();
        assert_eq!(res2, MigrationStatus::AlreadyCompleted);
        assert_eq!(std::fs::read(new_dir.join("new_session.txt")).unwrap(), b"USER_NEW_WORK");
    }

    #[test]
    fn test_case_5_partial_migration() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(old_dir.join("generations")).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"DB_DATA").unwrap();
        std::fs::write(old_dir.join("generations").join("gen1.wav"), b"GEN1").unwrap();

        // Simulate partial migration: only voicebox.db was copied, marker was not created
        std::fs::create_dir_all(&new_dir).unwrap();
        std::fs::write(new_dir.join("voicebox.db"), b"DB_DATA").unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir).expect("resume should succeed");
        assert!(matches!(res, MigrationStatus::Migrated { .. }));
        assert!(new_dir.join("generations").join("gen1.wav").exists());
        assert!(new_dir.join(MIGRATION_MARKER_FILENAME).exists());
    }

    #[test]
    fn test_case_6_same_destination_file_exists_identical() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::create_dir_all(&new_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"IDENTICAL_CONTENT").unwrap();
        std::fs::write(new_dir.join("voicebox.db"), b"IDENTICAL_CONTENT").unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir).expect("should succeed");
        assert!(matches!(res, MigrationStatus::Migrated { .. }));
        assert_eq!(std::fs::read(new_dir.join("voicebox.db")).unwrap(), b"IDENTICAL_CONTENT");
    }

    #[test]
    fn test_case_7_destination_contains_different_file() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::create_dir_all(&new_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"OLD_CONTENT").unwrap();
        std::fs::write(new_dir.join("voicebox.db"), b"DIFFERENT_NEW_CONTENT").unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir);
        assert!(res.is_err(), "Must error on destination conflict");
        match res.unwrap_err() {
            MigrationError::DestinationConflict(p) => {
                assert!(p.ends_with("voicebox.db"));
            }
            other => panic!("Expected DestinationConflict, got {:?}", other),
        }
        // Verify destination file was NOT overwritten
        assert_eq!(std::fs::read(new_dir.join("voicebox.db")).unwrap(), b"DIFFERENT_NEW_CONTENT");
        // Verify source was NOT altered
        assert_eq!(std::fs::read(old_dir.join("voicebox.db")).unwrap(), b"OLD_CONTENT");
    }

    #[test]
    fn test_case_8_source_unavailable_read_error() {
        let base = tempdir().unwrap();
        let old_file = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        // Create a regular file at old_file path so source exists, but read_dir fails
        std::fs::write(&old_file, b"REGULAR_FILE_NOT_DIR").unwrap();

        let res = migrate_data_dir(&old_file, &new_dir);
        assert!(res.is_err(), "Expected error when source cannot be read as dir");
        match res.unwrap_err() {
            MigrationError::SourceReadError(msg) => {
                assert!(!msg.is_empty());
            }
            other => panic!("Expected SourceReadError, got {:?}", other),
        }

        // Verify source file was not modified or deleted
        assert!(old_file.exists());
        assert_eq!(std::fs::read(&old_file).unwrap(), b"REGULAR_FILE_NOT_DIR");
    }

    #[test]
    fn test_case_9_migration_interrupted_before_completion_marker() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"MY_DB_DATA").unwrap();

        // Create an in_progress marker in new_dir
        std::fs::create_dir_all(&new_dir).unwrap();
        let in_progress = MigrationMarker {
            migration_version: HODUSON_DATA_MIGRATION_VERSION,
            source_identifier: OLD_BUNDLE_IDENTIFIER.to_string(),
            target_identifier: NEW_BUNDLE_IDENTIFIER.to_string(),
            started_at: "2026-10-02T10:00:00Z".to_string(),
            completed_at: None,
            status: "in_progress".to_string(),
            files_copied: 0,
            total_bytes: 0,
            db_sha256: None,
        };
        std::fs::write(
            new_dir.join(MIGRATION_MARKER_FILENAME),
            serde_json::to_string(&in_progress).unwrap(),
        )
        .unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir).expect("should resume and complete");
        assert!(matches!(res, MigrationStatus::Migrated { .. }));

        let final_marker: MigrationMarker = serde_json::from_str(
            &std::fs::read_to_string(new_dir.join(MIGRATION_MARKER_FILENAME)).unwrap(),
        )
        .unwrap();
        assert_eq!(final_marker.status, "completed");
        assert!(final_marker.completed_at.is_some());
    }

    #[test]
    fn test_case_10_source_database_active_wal_or_shm() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"SQLITE_DATA").unwrap();
        std::fs::write(old_dir.join("voicebox.db-wal"), b"WAL_TRANSACTION_DATA").unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir);
        assert!(res.is_err());
        match res.unwrap_err() {
            MigrationError::SourceDatabaseActive(msg) => {
                assert!(msg.contains("voicebox.db-wal"));
            }
            other => panic!("Expected SourceDatabaseActive, got {:?}", other),
        }

        // Target DB must NOT be copied
        assert!(!new_dir.join("voicebox.db").exists());
        // Completed marker must NOT be created
        assert!(!new_dir.join(MIGRATION_MARKER_FILENAME).exists());
        // Source files must be unchanged
        assert_eq!(std::fs::read(old_dir.join("voicebox.db")).unwrap(), b"SQLITE_DATA");
        assert_eq!(std::fs::read(old_dir.join("voicebox.db-wal")).unwrap(), b"WAL_TRANSACTION_DATA");

        // Also test SHM guard
        let base_shm = tempdir().unwrap();
        let old_shm_dir = base_shm.path().join("sh.voicebox.app");
        let new_shm_dir = base_shm.path().join("io.github.hoduson.voicestudio");
        std::fs::create_dir_all(&old_shm_dir).unwrap();
        std::fs::write(old_shm_dir.join("voicebox.db"), b"SQLITE_DATA").unwrap();
        std::fs::write(old_shm_dir.join("voicebox.db-shm"), b"SHM_DATA").unwrap();

        let res_shm = migrate_data_dir(&old_shm_dir, &new_shm_dir);
        assert!(res_shm.is_err());
        match res_shm.unwrap_err() {
            MigrationError::SourceDatabaseActive(msg) => {
                assert!(msg.contains("voicebox.db-shm"));
            }
            other => panic!("Expected SourceDatabaseActive, got {:?}", other),
        }
        assert!(!new_shm_dir.join("voicebox.db").exists());
        assert_eq!(std::fs::read(old_shm_dir.join("voicebox.db-shm")).unwrap(), b"SHM_DATA");
    }

    #[test]
    fn test_case_11_cannot_write_in_progress_marker() {
        let base = tempdir().unwrap();
        let old_dir = base.path().join("sh.voicebox.app");
        let new_dir = base.path().join("io.github.hoduson.voicestudio");

        std::fs::create_dir_all(&old_dir).unwrap();
        std::fs::write(old_dir.join("voicebox.db"), b"SQLITE_DATA").unwrap();

        // Create target directory, and create migration_marker.json as a DIRECTORY so write fails
        let marker_dir = new_dir.join(MIGRATION_MARKER_FILENAME);
        std::fs::create_dir_all(&marker_dir).unwrap();

        let res = migrate_data_dir(&old_dir, &new_dir);
        assert!(res.is_err());
        match res.unwrap_err() {
            MigrationError::Io(_) => {}
            other => panic!("Expected Io error when writing marker, got {:?}", other),
        }

        // Critical data must NOT be copied
        assert!(!new_dir.join("voicebox.db").exists());
        // Source data must be untouched
        assert_eq!(std::fs::read(old_dir.join("voicebox.db")).unwrap(), b"SQLITE_DATA");
    }

    #[test]
    #[ignore]
    fn run_real_migration_harness() {
        let appdata = std::env::var("APPDATA").expect("APPDATA must be set");
        let old_dir = PathBuf::from(&appdata).join(OLD_BUNDLE_IDENTIFIER);
        let new_dir = PathBuf::from(&appdata).join(NEW_BUNDLE_IDENTIFIER);

        println!("Running real migration harness:");
        println!("  Source: {:?}", old_dir);
        println!("  Target: {:?}", new_dir);

        let res = migrate_data_dir(&old_dir, &new_dir).expect("Real migration failed");
        println!("RESULT: {:?}", res);
    }
}
