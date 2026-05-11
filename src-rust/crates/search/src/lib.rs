//! # Asimov Search — Semantic Desktop Search Engine
//!
//! Fast, intelligent file search with semantic understanding.
//!
//! ## Features
//!
//! - **Blazing fast**: Sub-10ms query response for cached results
//! - **Semantic search**: Natural language queries like "that doc about coffee"
//! - **Incremental indexing**: Watches file system for changes
//! - **Privacy-first**: All data stored locally
//!
//! ## Usage
//!
//! ```rust,no_run
//! use asimov_search::{SearchEngine, SearchResult};
//!
//! # async fn example() -> anyhow::Result<()> {
//! let engine = SearchEngine::new(Default::default()).await?;
//! engine.index("/home/user").await?;
//!
//! let results = engine.search("meeting notes from last week").await?;
//! for result in results {
//!     println!("{}: {}", result.score, result.path.display());
//! }
//! # Ok(())
//! # }
//! ```

pub mod indexer;
pub mod semantic;
pub mod metadata;
pub mod ranker;
pub mod parser;
pub mod cache;
pub mod mascot;

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::metadata::MetadataIndex;
use crate::ranker::Ranker;
use crate::parser::QueryParser;

/// Configuration for the search engine
#[derive(Debug, Clone)]
pub struct Config {
    /// Base directory for index storage
    pub index_path: PathBuf,
    /// Paths to index (supports glob patterns)
    pub watched_paths: Vec<PathBuf>,
    /// Patterns to exclude from indexing
    pub excluded_patterns: Vec<String>,
    /// Maximum file size to index (bytes)
    pub max_file_size: u64,
    /// Embedding model to use
    pub embedder: EmbedderType,
    /// Batch size for embedding
    pub batch_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            index_path: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".asimov/search"),
            watched_paths: vec![dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))],
            excluded_patterns: vec![
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/target/**".to_string(),
                "**/.cache/**".to_string(),
                "**/*.lock".to_string(),
            ],
            max_file_size: 50 * 1024 * 1024, // 50MB
            embedder: EmbedderType::MiniLM,
            batch_size: 128,
        }
    }
}

/// Embedding model types
#[derive(Debug, Clone, Copy)]
pub enum EmbedderType {
    /// all-MiniLM-L6-v2: Fast, 384-dim, 22M params
    MiniLM,
    /// bge-small-en-v1.5: Faster, 1024-dim, 33M params
    BGE,
    /// e5-base-v2: Higher quality, 768-dim, 278M params
    E5,
}

impl Default for EmbedderType {
    fn default() -> Self {
        Self::MiniLM
    }
}

/// A search result with score and metadata
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Path to the file
    pub path: PathBuf,
    /// Relevance score (0.0 - 1.0)
    pub score: f32,
    /// File type (code, doc, image, etc.)
    pub file_type: FileType,
    /// Matched snippet with highlighting
    pub snippet: String,
    /// Last modified timestamp
    pub modified: chrono::DateTime<chrono::Utc>,
}

/// File type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Code,
    Document,
    Image,
    Audio,
    Video,
    Config,
    Other,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            // Code
            "rs" | "py" | "js" | "ts" | "go" | "java" | "c" | "cpp" | "h" | "cs"
            | "rb" | "php" | "swift" | "kt" | "scala" | "ex" | "exs" | "hs"
            | "ml" | "clj" | "cljs" | "erl" | "ex" | "fs" | "fsx" | "vue"
            | "svelte" | "jsx" | "tsx" | "css" | "scss" | "sass" | "less"
            | "html" | "htm" | "xml" | "json" | "yaml" | "yml" | "toml"
            | "md" | "markdown" | "txt" | "rst" | "tex" | "sql" | "sh"
            | "bash" | "zsh" | "fish" | "ps1" | "bat" | "cmd" | "awk"
            | "sed" | "vim" | "lua" | "r" | "jl" | "zig" | "nim" | "v"
            | "vlang" | "odin" | "gleam" => FileType::Code,
            
            // Documents
            "pdf" | "doc" | "docx" | "odt" | "rtf" | "pages" | "xls"
            | "xlsx" | "ods" | "csv" | "ppt" | "pptx" | "odp" => FileType::Document,
            
            // Images
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp"
            | "ico" | "tiff" | "tif" | "psd" | "raw" | "heic" => FileType::Image,
            
            // Audio
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" | "wma"
            | "opus" | "aiff" => FileType::Audio,
            
            // Video
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm"
            | "m4v" | "mpeg" | "mpg" => FileType::Video,
            
            // Config
            "env" | "ini" | "cfg" | "conf" | "config" | "properties"
            | "editorconfig" | "prettierrc" | "eslintrc" | "gitignore"
            | "dockerignore" | "lock" => FileType::Config,
            
            _ => FileType::Other,
        }
    }
}

/// Main search engine
pub struct SearchEngine {
    config: Config,
    metadata_index: Arc<RwLock<MetadataIndex>>,
    ranker: Ranker,
    query_parser: QueryParser,
    embedder: semantic::Embedder,
}

impl SearchEngine {
    /// Create a new search engine
    pub async fn new(config: Config) -> Result<Self> {
        // Create index directory
        tokio::fs::create_dir_all(&config.index_path).await?;
        
        // Initialize metadata index
        let db_path = config.index_path.join("metadata.db");
        let metadata_index = MetadataIndex::new(&db_path).await?;
        
        let ranker = Ranker::new();
        let query_parser = QueryParser::new();
        let embedder = semantic::Embedder::new(config.embedder, &config.index_path).await?;
        
        Ok(Self {
            config,
            metadata_index: Arc::new(RwLock::new(metadata_index)),
            ranker,
            query_parser,
            embedder,
        })
    }
    
    /// Index a directory
    pub async fn index(&self, path: &Path) -> Result<IndexStats> {
        let walker = indexer::FileWalker::new(&self.config.excluded_patterns);
        let files = walker.walk(path).await?;
        
        let total = files.len();
        let mut indexed = 0;
        let mut skipped = 0;
        
        for file in files {
            if file.size > self.config.max_file_size {
                skipped += 1;
                continue;
            }
            
            // Extract content and metadata
            let metadata = indexer::extract_metadata(&file.path).await?;
            
            // Store in metadata index
            {
                let mut index = self.metadata_index.write().await;
                index.add_file(&metadata).await?;
            }
            
            indexed += 1;
        }
        
        Ok(IndexStats {
            total,
            indexed,
            skipped,
        })
    }
    
    /// Search for files
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // Parse query
        let parsed = self.query_parser.parse(query);
        
        // Get candidate files from metadata index
        let candidates = {
            let index = self.metadata_index.read().await;
            index.query(&parsed).await?
        };
        
        // Score and rank results
        let results = self.ranker.rank(candidates, &parsed).await?;
        
        Ok(results)
    }
    
    /// Get index statistics
    pub async fn stats(&self) -> Result<crate::metadata::IndexStats> {
        let index = self.metadata_index.read().await;
        index.stats().await
    }
}

/// Statistics from an indexing operation
#[derive(Debug)]
pub struct IndexStats {
    pub total: usize,
    pub indexed: usize,
    pub skipped: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_file_type_detection() {
        assert_eq!(FileType::from_extension("rs"), FileType::Code);
        assert_eq!(FileType::from_extension("PDF"), FileType::Document);
        assert_eq!(FileType::from_extension("PNG"), FileType::Image);
        assert_eq!(FileType::from_extension("xyz"), FileType::Other);
    }
    
    #[test]
    fn test_query_parsing() {
        let parser = QueryParser::new();
        
        let parsed = parser.parse("meeting notes");
        assert!(!parsed.has_semantic_intent());
        
        let parsed = parser.parse("something about coffee and meetings");
        assert!(parsed.has_semantic_intent());
    }
}
