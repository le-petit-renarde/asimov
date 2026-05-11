//! Result Ranker — Hybrid scoring

use crate::metadata::FileMetadata;
use crate::parser::ParsedQuery;
use crate::SearchResult;
use chrono::{DateTime, Utc};

/// Result ranker combining multiple signals
pub struct Ranker {
    // Boost parameters
    recency_boost_days: u32,
}

impl Ranker {
    pub fn new() -> Self {
        Self {
            recency_boost_days: 7,
        }
    }
    
    /// Rank files based on query
    pub async fn rank(
        &self,
        files: Vec<FileMetadata>,
        parsed: &ParsedQuery,
    ) -> anyhow::Result<Vec<SearchResult>> {
        let now = Utc::now().timestamp();
        let query_lower = parsed.original.to_lowercase();
        
        let mut scored: Vec<_> = files
            .into_iter()
            .map(|file| {
                // Base score from recency
                let days_ago = (now - file.modified) / 86400;
                let recency_score = if days_ago < self.recency_boost_days as i64 {
                    1.0 - (days_ago as f32 / self.recency_boost_days as f32) * 0.3
                } else {
                    0.7
                };
                
                // Name match bonus
                let name_lower = file.name.to_lowercase();
                let name_match = if name_lower.contains(&query_lower) {
                    0.4
                } else if name_lower.split(|c: char| !c.is_alphanumeric())
                    .any(|word| word.len() > 2 && query_lower.contains(word))
                {
                    0.2
                } else {
                    0.0
                };
                
                // Path match bonus
                let path_lower = file.path.to_string_lossy().to_lowercase();
                let path_match = if path_lower.contains(&query_lower) {
                    0.1
                } else {
                    0.0
                };
                
                // File type relevance boost
                let type_boost = match file.file_type {
                    crate::FileType::Code => 0.1,
                    crate::FileType::Document => 0.15,
                    crate::FileType::Config => 0.15,
                    crate::FileType::Image => 0.05,
                    crate::FileType::Audio => 0.05,
                    crate::FileType::Video => 0.05,
                    crate::FileType::Other => 0.0,
                };
                
                // Size penalty for very large files (they're often binaries)
                let size_penalty = if file.size > 10_000_000 { -0.1 } else { 0.0 };
                
                let total_score = (recency_score + name_match + path_match + type_boost + size_penalty)
                    .clamp(0.0, 1.0);
                
                (file, total_score)
            })
            .collect();
        
        // Sort by score descending
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Convert to SearchResults
        let results = scored
            .into_iter()
            .take(20)
            .map(|(file, score)| {
                SearchResult {
                    path: file.path,
                    score,
                    file_type: file.file_type,
                    snippet: generate_snippet(&file),
                    modified: DateTime::from_timestamp(file.modified, 0)
                        .unwrap_or_else(|| Utc::now()),
                }
            })
            .collect();
        
        Ok(results)
    }
}

fn generate_snippet(file: &FileMetadata) -> String {
    let name = &file.name;
    let parent = &file.parent;
    
    if parent.is_empty() {
        name.clone()
    } else {
        format!("{}/{}", parent, name)
    }
}

impl Default for Ranker {
    fn default() -> Self {
        Self::new()
    }
}
