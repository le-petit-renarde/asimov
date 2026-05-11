//! File Indexer — Parallel file system walking and content extraction

use anyhow::Result;
use glob::Pattern;
use sha2::{Sha256, Digest};
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;

use crate::metadata::FileMetadata;
use crate::FileType;

/// File walker with ignore patterns
pub struct FileWalker {
    excluded_patterns: Vec<Pattern>,
}

impl FileWalker {
    pub fn new(excluded: &[String]) -> Self {
        let excluded_patterns: Vec<Pattern> = excluded
            .iter()
            .filter_map(|p| Pattern::new(p).ok())
            .collect();
        
        Self { excluded_patterns }
    }
    
    /// Walk a directory and return file info
    pub async fn walk(&self, root: &Path) -> Result<Vec<FileInfo>> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip directories
            if entry.file_type().is_dir() {
                continue;
            }
            
            // Skip excluded patterns
            let path_str = path.to_string_lossy();
            if self.is_excluded(&path_str) {
                continue;
            }
            
            // Get metadata
            if let Ok(metadata) = entry.metadata() {
                let info = FileInfo {
                    path: path.to_path_buf(),
                    size: metadata.len(),
                    modified: metadata.modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0),
                    accessed: metadata.accessed()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0),
                };
                files.push(info);
            }
        }
        
        Ok(files)
    }
    
    fn is_excluded(&self, path: &str) -> bool {
        self.excluded_patterns.iter().any(|p| p.matches(path))
    }
}

/// Basic file information
#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub modified: i64,
    pub accessed: i64,
}

/// Extract metadata from a file
pub async fn extract_metadata(path: &Path) -> Result<FileMetadata> {
    let metadata = fs::metadata(path).await?;
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let parent = path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let extension = path
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_default();
    
    // Compute content hash (first 64KB only for speed)
    let content_hash = compute_partial_hash(path).await?;
    
    Ok(FileMetadata {
        path: path.to_path_buf(),
        name,
        parent,
        size: metadata.len(),
        modified: metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0),
        accessed: metadata.accessed()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0),
        file_type: FileType::from_extension(&extension),
        mime_type: None,
        content_hash,
        chunk_count: 0,
        indexed_at: chrono::Utc::now().timestamp(),
    })
}

/// Compute a fast partial hash of a file
async fn compute_partial_hash(path: &Path) -> Result<String> {
    let file = fs::File::open(path).await?;
    use tokio::io::AsyncReadExt;
    
    let mut reader = file;
    let mut buffer = vec![0u8; 65536]; // First 64KB
    let n = reader.read(&mut buffer).await?;
    buffer.truncate(n);
    
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();
    
    Ok(hex::encode(result))
}
