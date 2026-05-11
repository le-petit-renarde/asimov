//! Query Parser — Understanding user intent

use regex::Regex;

/// Parsed query with filters extracted
#[derive(Debug, Clone)]
pub struct ParsedQuery {
    /// Original query
    pub original: String,
    /// Tokens for literal matching
    pub literal_tokens: Option<Vec<String>>,
    /// Semantic part of query (for embedding)
    pub semantic_part: Option<String>,
    /// Time filter (if any)
    pub time_filter: Option<TimeFilter>,
    /// File type filter (if any)
    pub type_filter: Option<String>,
    /// Scope filter (if any)
    pub scope_filter: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TimeFilter {
    Today,
    Yesterday,
    DaysAgo(u32),
    ThisWeek,
    ThisMonth,
}

/// Query parser
pub struct QueryParser {
    // Regex patterns for filter detection
    type_pattern: Regex,
    time_patterns: Vec<Regex>,
}

impl QueryParser {
    pub fn new() -> Self {
        let type_pattern = Regex::new(r"(?i)\b(type:)(pdf|doc|code|md|img|image|audio|video)\b").unwrap();
        
        let time_patterns = vec![
            Regex::new(r"(?i)\btoday\b").unwrap(),
            Regex::new(r"(?i)\byesterday\b").unwrap(),
            Regex::new(r"(?i)\bthis week\b").unwrap(),
            Regex::new(r"(?i)\bthis month\b").unwrap(),
            Regex::new(r"(?i)\b(\d+)d\b").unwrap(),
            Regex::new(r"(?i)\b(\d+)w\b").unwrap(),
        ];
        
        Self {
            type_pattern,
            time_patterns,
        }
    }
    
    /// Parse a query string
    pub fn parse(&self, query: &str) -> ParsedQuery {
        let original = query.to_string();
        let mut query = query.to_string();
        let mut time_filter = None;
        let mut type_filter = None;
        
        // Extract type filter
        if let Some(caps) = self.type_pattern.captures(&query) {
            if let Some(t) = caps.get(2) {
                type_filter = Some(t.as_str().to_lowercase());
                query = self.type_pattern.replace(&query, "").to_string();
            }
        }
        
        // Extract time filter
        for pattern in &self.time_patterns {
            if let Some(caps) = pattern.captures(&query) {
                let matched = caps.get(0).map(|m| m.as_str()).unwrap_or("");
                time_filter = self.parse_time_filter(matched);
                if time_filter.is_some() {
                    query = pattern.replace(&query, "").to_string();
                    break;
                }
            }
        }
        
        // Clean up query
        query = query.trim().to_string();
        query = Regex::new(r"\s+")
            .unwrap()
            .replace_all(&query, " ")
            .to_string();
        
        // Determine if semantic or literal
        let has_semantic_intent = self.detect_semantic_intent(&query);
        
        ParsedQuery {
            original: original.clone(),
            literal_tokens: if query.is_empty() { None } else { Some(vec![query.clone()]) },
            semantic_part: if has_semantic_intent { Some(query) } else { None },
            time_filter,
            type_filter,
            scope_filter: None,
        }
    }
    
    fn parse_time_filter(&self, matched: &str) -> Option<TimeFilter> {
        let lower = matched.to_lowercase();
        if lower == "today" {
            Some(TimeFilter::Today)
        } else if lower == "yesterday" {
            Some(TimeFilter::Yesterday)
        } else if lower == "this week" {
            Some(TimeFilter::ThisWeek)
        } else if lower == "this month" {
            Some(TimeFilter::ThisMonth)
        } else if let Some(d) = Regex::new(r"(\d+)d")
            .ok()?
            .captures(&lower)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
        {
            Some(TimeFilter::DaysAgo(d))
        } else if let Some(w) = Regex::new(r"(\d+)w")
            .ok()?
            .captures(&lower)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
        {
            Some(TimeFilter::DaysAgo(w * 7))
        } else {
            None
        }
    }
    
    /// Detect if a query has semantic intent
    fn detect_semantic_intent(&self, query: &str) -> bool {
        // Queries with natural language patterns suggest semantic intent
        let semantic_indicators = [
            "about", "related", "similar", "like", "containing",
            "that", "something", "things", "stuff", "document",
            "file", "thing", "maybe", "perhaps", "involving",
        ];
        
        let words: Vec<&str> = query.to_lowercase().split_whitespace().collect();
        let indicator_count = words.iter()
            .filter(|w| semantic_indicators.contains(w))
            .count();
        
        // If contains natural language connectors, likely semantic
        if indicator_count > 0 {
            return true;
        }
        
        // Long queries (>3 words) without file extensions are likely semantic
        if words.len() > 3 && !query.contains('.') && !query.contains('/') {
            return true;
        }
        
        // Contains phrases suggesting understanding needed
        if query.len() > 30 {
            return true;
        }
        
        false
    }
}

impl Default for QueryParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ParsedQuery {
    pub fn has_semantic_intent(&self) -> bool {
        self.semantic_part.is_some()
    }
}
