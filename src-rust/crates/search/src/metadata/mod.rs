//! Metadata Index — SQLite-based fast file indexing

use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::parser::ParsedQuery;
use crate::{FileType, SearchResult};

/// Metadata for a single file
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub name: String,
    pub parent: String,
    pub size: u64,
    pub modified: i64,
    pub accessed: i64,
    pub file_type: FileType,
    pub mime_type: Option<String>,
    pub content_hash: String,
    pub chunk_count: usize,
    pub indexed_at: i64,
}

/// SQLite-based metadata index
pub struct MetadataIndex {
    conn: Connection,
}

impl MetadataIndex {
    /// Create a new metadata index
    pub async fn new(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA cache_size = -64000;  -- 64MB cache
            
            CREATE TABLE IF NOT EXISTS files (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                parent TEXT NOT NULL,
                size INTEGER NOT NULL,
                modified INTEGER NOT NULL,
                accessed INTEGER NOT NULL,
                file_type TEXT NOT NULL,
                mime_type TEXT,
                content_hash TEXT NOT NULL,
                chunk_count INTEGER DEFAULT 0,
                indexed_at INTEGER NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_path ON files(path);
            CREATE INDEX IF NOT EXISTS idx_modified ON files(modified DESC);
            CREATE INDEX IF NOT EXISTS idx_type ON files(file_type);
            CREATE INDEX IF NOT EXISTS idx_name ON files(name);
            
            CREATE VIRTUAL TABLE IF NOT EXISTS files_fts USING fts5(
                name, 
                path,
                content='files',
                content_rowid='id'
            );
            "#,
        )?;
        
        Ok(Self { conn })
    }
    
    /// Add a file to the index
    pub async fn add_file(&mut self, meta: &FileMetadata) -> Result<()> {
        let file_type = format!("{:?}", meta.file_type).to_lowercase();
        
        self.conn.execute(
            r#"
            INSERT OR REPLACE INTO files 
            (path, name, parent, size, modified, accessed, file_type, mime_type, content_hash, chunk_count, indexed_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            params![
                meta.path.to_string_lossy(),
                meta.name,
                meta.parent,
                meta.size as i64,
                meta.modified,
                meta.accessed,
                file_type,
                meta.mime_type,
                meta.content_hash,
                meta.chunk_count as i64,
                meta.indexed_at,
            ],
        )?;
        
        // Update FTS index
        self.conn.execute(
            "INSERT INTO files_fts(rowid, name, path) VALUES (last_rowid, ?1, ?2)",
            params![meta.name, meta.path.to_string_lossy()],
        )?;
        
        Ok(())
    }
    
    /// Update a file in the index
    pub async fn update_file(&mut self, meta: &FileMetadata) -> Result<()> {
        self.add_file(meta).await
    }
    
    /// Remove a file from the index
    pub async fn remove_file(&mut self, path: &Path) -> Result<()> {
        self.conn.execute(
            "DELETE FROM files WHERE path = ?1",
            params![path.to_string_lossy()],
        )?;
        Ok(())
    }
    
    /// Query files matching the parsed query
    pub async fn query(&self, parsed: &ParsedQuery) -> Result<Vec<FileMetadata>> {
        let mut files = Vec::new();
        
        // If literal query, use FTS
        if !parsed.has_semantic_intent() {
            if let Some(tokens) = &parsed.literal_tokens {
                let fts_query = tokens.join(" AND ");
                let mut stmt = self.conn.prepare(
                    r#"
                    SELECT f.* FROM files f
                    JOIN files_fts fts ON f.id = fts.rowid
                    WHERE files_fts MATCH ?1
                    ORDER BY f.modified DESC
                    LIMIT 100
                    "#,
                )?;
                
                let rows = stmt.query_map(params![fts_query], |row| {
                    Ok(FileMetadata {
                        path: PathBuf::from(row.get::<_, String>(0)?),
                        name: row.get(1)?,
                        parent: row.get(2)?,
                        size: row.get::<_, i64>(3)? as u64,
                        modified: row.get(4)?,
                        accessed: row.get(5)?,
                        file_type: parse_file_type(&row.get::<_, String>(6)?),
                        mime_type: row.get(7)?,
                        content_hash: row.get(8)?,
                        chunk_count: row.get::<_, i64>(9)? as usize,
                        indexed_at: row.get(10)?,
                    })
                })?;
                
                for row in rows {
                    files.push(row?);
                }
            }
        } else {
            // For semantic queries, just get recent files
            let mut stmt = self.conn.prepare(
                r#"
                SELECT * FROM files
                ORDER BY modified DESC
                LIMIT 500
                "#,
            )?;
            
            let rows = stmt.query_map([], |row| {
                Ok(FileMetadata {
                    path: PathBuf::from(row.get::<_, String>(0)?),
                    name: row.get(1)?,
                    parent: row.get(2)?,
                    size: row.get::<_, i64>(3)? as u64,
                    modified: row.get(4)?,
                    accessed: row.get(5)?,
                    file_type: parse_file_type(&row.get::<_, String>(6)?),
                    mime_type: row.get(7)?,
                    content_hash: row.get(8)?,
                    chunk_count: row.get::<_, i64>(9)? as usize,
                    indexed_at: row.get(10)?,
                })
            })?;
            
            for row in rows {
                files.push(row?);
            }
        }
        
        Ok(files)
    }
    
    /// Get index statistics
    pub async fn stats(&self) -> Result<IndexStats> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM files",
            [],
            |row| row.get(0),
        )?;
        
        let total_size: i64 = self.conn.query_row(
            "SELECT SUM(size) FROM files",
            [],
            |row| row.get(0),
        )?;
        
        Ok(IndexStats {
            file_count: count as usize,
            total_size: total_size as u64,
        })
    }
}

fn parse_file_type(s: &str) -> FileType {
    match s {
        "code" => FileType::Code,
        "document" => FileType::Document,
        "image" => FileType::Image,
        "audio" => FileType::Audio,
        "video" => FileType::Video,
        "config" => FileType::Config,
        _ => FileType::Other,
    }
}

#[derive(Debug)]
pub struct IndexStats {
    pub file_count: usize,
    pub total_size: u64,
}
