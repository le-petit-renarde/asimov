# Asimov — Architecture Blueprint

> Semantic Desktop Assistant & General-Purpose AI Companion

---

## 1. Brand Rename: Asimov → Asimov

### 1.1 Files Requiring Rename

| Scope | Pattern | Action |
|-------|---------|--------|
| **Crates** | `asimov-*` | Rename to `asimov-*` in `Cargo.toml`, `Cargo.toml` per crate |
| **Binary** | `crates/cli/src/main.rs` | Update comments, string literals |
| **Build script** | `build.rs` | Update metadata strings |
| **README** | `README.md` | Full rebrand |
| **Public assets** | `public/` | Replace mascot images |
| **Source code** | All `*.rs` files | `sed` replace `asimov` → `asimov`, `Asimov` → `Asimov` |

### 1.2 Rename Execution Script

```bash
#!/bin/bash
# rename.sh — Execute once

# 1. Update workspace Cargo.toml
sed -i 's/asimov/asimov/g' src-rust/Cargo.toml
sed -i 's/Asimov/Asimov/g' src-rust/Cargo.toml

# 2. Rename crate directories
for crate in core api tools query tui commands mcp bridge buddy plugins acp cli; do
    mv "src-rust/crates/$crate/Cargo.toml" "src-rust/crates/$crate/Cargo.toml.bak" 2>/dev/null
    sed 's/asimov/asimov/g; s/Asimov/Asimov/g' "src-rust/crates/$crate/Cargo.toml.bak" > "src-rust/crates/$crate/Cargo.toml"
    rm "src-rust/crates/$crate/Cargo.toml.bak"
done

# 3. Update all Rust source files
find src-rust -name "*.rs" -exec sed -i 's/asimov/asimov/g; s/Asimov/Asimov/g; s/ASIMOV/ASIMOV/g' {} \;

# 4. Update build.rs
sed -i 's/Asimov/Asimov/g; s/asimov/asimov/g' src-rust/crates/cli/build.rs

# 5. Update README
sed -i 's/ASIMOV/ASIMOV/g; s/Asimov/Asimov/g; s/asimov/asimov/g' README.md

# 6. Update package metadata in root
sed -i 's/asimov/asimov/g; s/Asimov/Asimov/g; s/ASIMOV/ASIMOV/g' package.json 2>/dev/null

# 7. Git commit
git add -A && git commit -m "feat: rename Asimov → Asimov"
```

### 1.3 Mascot Branding

Replace in `public/`:
- `Ship.png` → **"Asimov Ship"** — A cozy starship/steam-punk observatory
- `Rustle.png` → **"Professor"** — Figure with glasses, pipe, holding a book
- `Pirate-Rustle.png` → Remove or repurpose
- `social-image.png` → Regenerated with Asimov branding

---

## 2. Semantic Desktop Search — "Astra"

### Design Philosophy

> **"Find anything in milliseconds, understand everything in context."**

Inspired by: Everything (Windows), Alfred (macOS), Silver Searcher (CLI speed), but with **semantic understanding**.

### 2.1 Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER QUERY                               │
│              "that doc about the Romanian roaster"               │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                     QUERY PARSER                                 │
│  • Intent detection (semantic vs literal)                        │
│  • Time filters: "last week", "yesterday"                        │
│  • File type filters: "pdf", "code", "image"                    │
│  • Scope filters: "in project X", "from email"                  │
└─────────────────────────┬───────────────────────────────────────┘
                          │
          ┌───────────────┼───────────────┐
          ▼               ▼               ▼
    ┌──────────┐   ┌──────────┐   ┌──────────┐
    │  METADATA │   │ SEMANTIC │   │  HYBRID  │
    │   INDEX   │   │   INDEX  │   │  MERGER  │
    │ (TinySQL) │   │  (USearch│   │  (ReRank)│
    │   <1ms    │   │  + Embed)│   │          │
    └──────────┘   └──────────┘   └──────────┘
          │               │               │
          └───────────────┼───────────────┘
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    RESULT RANKER                                 │
│  • Recency boost                                                │
│  • Access frequency                                             │
│  • Location relevance                                           │
│  • Semantic similarity                                          │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                     SNIPPET GENERATOR                            │
│  • Highlight matched terms                                      │
│  • Extract context preview                                      │
│  • Quick actions (open, copy path, share)                       │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 New Crate: `asimov-search`

```
src-rust/crates/search/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── indexer/           # File system walker + content extraction
    │   ├── walker.rs      # Parallel walk with ignore patterns
    │   ├── extractor.rs   # Text extraction (PDF, DOCX, MD, code)
    │   ├── chunker.rs     # Split large files into searchable chunks
    │   └── metadata.rs    # File metadata (size, dates, permissions)
    ├── semantic/          # Embedding-based search
    │   ├── embeddings.rs  # ONNX runtime + sentence-transformers
    │   ├── usearch.rs     # Vector index (USearch — 10x faster than FAISS)
    │   └── embedder.rs    # Model loading, batching, caching
    ├── metadata/          # SQLite-based metadata index
    │   ├── store.rs       # CRUD operations
    │   ├── query.rs       # Fast prefix/glob/time queries
    │   └── schema.rs      # Table definitions
    ├── ranker/            # Result ranking
    │   ├── scoring.rs     # BM25 + recency + frequency
    │   └── rerank.rs      # Cross-encoder re-ranking
    ├── parser/            # Query understanding
    │   ├── lexer.rs       # Tokenize query
    │   ├── filters.rs     # Parse time/type/scope filters
    │   └── intent.rs      # Detect literal vs semantic intent
    ├── cache/             # LRU cache for hot paths
    │   └── lru.rs
    └── cli.rs             # Standalone CLI for debugging
```

### 2.3 Key Technical Decisions

| Component | Choice | Rationale |
|-----------|--------|-----------|
| **Vector DB** | USearch | 10-100x faster than FAISS, pure Rust, no external deps |
| **Embeddings** | ONNX + `all-MiniLM-L6-v2` | 384-dim, 22M params, <50ms inference on CPU |
| **Metadata DB** | SQLite (rusqlite) | Already in workspace deps, ACID, WAL mode |
| **File extraction** | `pdf-extract`, `docx-rs`, `syntect` | Code highlighting, document parsing |
| **Async I/O** | Tokio | Already used in workspace |
| **Caching** | In-memory LRU | Hot paths (recent queries, common dirs) |

### 2.4 Performance Targets

| Metric | Target |
|--------|--------|
| **Cold index** | 1M files in <10 minutes |
| **Query latency** | <10ms for cached, <50ms for first semantic query |
| **Memory** | <500MB for 1M file index |
| **Index update** | Incremental, <100ms per file change |
| **Embedding batch** | 128 chunks in <200ms |

### 2.5 Index Schema

```sql
-- Metadata index (SQLite)
CREATE TABLE files (
    id          INTEGER PRIMARY KEY,
    path        TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    parent      TEXT NOT NULL,
    size        INTEGER NOT NULL,
    modified    INTEGER NOT NULL,  -- Unix timestamp
    accessed    INTEGER NOT NULL,
    file_type   TEXT NOT NULL,     -- 'code', 'doc', 'image', 'other'
    mime_type   TEXT,
    content_hash TEXT NOT NULL,    -- For change detection
    chunk_count INTEGER DEFAULT 0,
    indexed_at  INTEGER NOT NULL
);

CREATE INDEX idx_path ON files(path);
CREATE INDEX idx_modified ON files(modified DESC);
CREATE INDEX idx_type ON files(file_type);
CREATE VIRTUAL TABLE files_fts USING fts5(name, path, content);

-- Semantic index (USearch)
-- Stored in: ~/.asimov/search/vectors.usearch
```

### 2.6 Query Flow (Pseudocode)

```rust
async fn search(query: &str) -> Vec<SearchResult> {
    // 1. Parse query
    let parsed = QueryParser::new(query).parse();
    
    // 2. Metadata pre-filter (sub-millisecond)
    let candidates = if parsed.has_semantic_intent() {
        // Start with broader set for semantic
        metadata_store.scan_all()
    } else {
        // Strict filter for literal queries
        metadata_store.query(&parsed.literal_filters()).await?
    };
    
    // 3. Semantic embedding (lazy, only if needed)
    let query_embedding = if parsed.has_semantic_intent() {
        Some(embedder.embed(&parsed.semantic_part()).await?)
    } else {
        None
    };
    
    // 4. Hybrid scoring
    let mut results: Vec<ScoredResult> = candidates
        .into_iter()
        .map(|file| {
            let bm25 = bm25_score(&file, &parsed.tokens());
            let semantic = query_embedding
                .as_ref()
                .map(|emb| usearch.search(emb, file.chunk_vectors()));
            let recency = recency_boost(file.modified);
            let frequency = access_frequency(&file);
            
            Score { bm25, semantic, recency, frequency }
        })
        .collect();
    
    // 5. Top-K with reranking
    results.sort_by(|a, b| b.total_score().partial_cmp(&a.total_score()).unwrap());
    results.truncate(20);
    
    // 6. Generate snippets
    results.into_iter()
        .map(|r| generate_snippet(&r, query))
        .collect()
}
```

### 2.7 File Watching & Incremental Updates

```rust
// Use `notify` crate for cross-platform file watching
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

fn start_watcher(index: Arc<SearchIndex>) {
    let watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, _>| {
            match res {
                Ok(event) => {
                    let mut idx = index.lock();
                    for path in event.paths {
                        match event.kind {
                            EventKind::Create(_) => idx.add_file(&path),
                            EventKind::Modify(_) => idx.update_file(&path),
                            EventKind::Remove(_) => idx.remove_file(&path),
                            _ => {}
                        }
                    }
                }
                Err(e) => warn!("Watch error: {:?}", e),
            }
        },
        Config::default(),
    ).unwrap();
    
    watcher.watch(home_dir(), RecursiveMode::Recursive).unwrap();
}
```

### 2.8 CLI Integration

```bash
# New command in Asimov
asimov search "Romanian roaster meeting notes"
asimov search --type pdf "quarterly report"
asimov search --since 7d "rust async patterns"
asimov index --rebuild  # Full reindex
asimov index --status   # Show index stats
```

### 2.9 Embedding Model Strategy

```rust
// Default: all-MiniLM-L6-v2 (ONNX)
// Alternatives (configurable):
//   - bge-small-en-v1.5 (faster, 256-dim)
//   - nomic-embed-text-v1.5 (larger context)
//   - e5-base-v2 (best quality, slower)

pub enum EmbedderModel {
    MiniLM,    // Default: 384-dim, 22M params
    BGE,       // Fast: 1024-dim, 33M params
    E5,        // Quality: 768-dim, 278M params
}
```

---

## 3. Proactive Context Engine — "Muse"

### 3.1 Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      MUSE — Context Engine                       │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────┐        │
│  │   WINDOW     │   │   FILES      │   │   CALENDAR   │        │
│  │   WATCHER    │   │   WATCHER    │   │   (optional) │        │
│  └──────┬───────┘   └──────┬───────┘   └──────┬───────┘        │
│         │                  │                  │                 │
│         └──────────────────┼──────────────────┘                 │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │                   CONTEXT BUFFER                         │    │
│  │  • Active window (app, title, URL)                      │    │
│  │  • Open files (path, language, recent edits)            │    │
│  │  • Clipboard history (text, images)                     │    │
│  │  • Time + location context                               │    │
│  └─────────────────────────┬───────────────────────────────┘    │
│                            │                                     │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │               PATTERN LEARNER                            │    │
│  │  • Time-based patterns (morning standup, code review)   │    │
│  │  • Workflow sequences (commit → push → PR)               │    │
│  │  • Attention hotspots (what you focus on)                │    │
│  └─────────────────────────┬───────────────────────────────┘    │
│                            │                                     │
│                            ▼                                     │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │               CONTEXT PREDICTOR                           │    │
│  │  • Pre-load relevant files/docs based on current context  │    │
│  │  • Suggest next actions based on learned patterns        │    │
│  │  • Alert on deadlines, follow-ups, interrupts            │    │
│  └─────────────────────────────────────────────────────────┘    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 3.2 New Crate: `asimov-context`

```
src-rust/crates/context/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── monitors/
    │   ├── window.rs      # Active window tracking (xdotool, xprop)
    │   ├── files.rs       # File access monitoring (inotify)
    │   ├── clipboard.rs   # Clipboard history
    │   └── keyboard.rs    # Keystroke patterns
    ├── buffer.rs          # In-memory context buffer
    ├── patterns.rs        # Pattern learning & storage
    ├── predict.rs         # Context prediction logic
    ├── hooks.rs           # Integration points with main agent
    └── config.rs          # Privacy controls, learning settings
```

### 3.3 Privacy Controls

```rust
pub struct PrivacyConfig {
    pub window_tracking: bool,      // Default: true
    pub file_tracking: bool,        // Default: true
    pub clipboard_history: bool,    // Default: true
    pub learning_enabled: bool,     // Default: true
    pub exclude_patterns: Vec<Glob>, // Never track these
    pub sensitive_apps: Vec<String>, // Browsers, password managers
}
```

---

## 4. Mascot Design: "Professor"

### 4.1 Concept

A distinguished figure — think classic detective or scholar — with:
- **Round glasses** (wire-frame style)
- **Curved pipe** (gentleman scholar vibe)
- **Cozy attire** (sweater vest, collared shirt)
- **Pose**: Thoughtful, perhaps holding a magnifying glass or book
- **Style**: Minimalist, warm colors, slightly retro

### 4.2 Visual Direction

```
┌────────────────────────────────────────────┐
│                                            │
│         ┌─────────────────────┐            │
│         │    ◯◯               │            │
│         │   (●●)  ← glasses   │            │
│         │    ‿‿   ← mustache  │            │
│         │      └────┐          │            │
│         │    ___|__█___       │            │
│         │   [███████████]     │            │
│         │    │ PROFESSOR │    │            │
│         │    └───────────┘     │            │
│         │                      │            │
│         │  🕯️ (optional candle) │            │
│         └─────────────────────┘            │
│                                            │
│    "Omniscient helper, always thinking"    │
└────────────────────────────────────────────┘
```

### 4.3 Implementation Options

| Option | Pros | Cons |
|--------|------|------|
| **Pixel art (PNG)** | Fast, retro charm, easy animation | Limited detail |
| **SVG** | Scalable, clean, animatable | No built-in texture |
| **ASCII art** | Terminal-native | Limited visual appeal |
| **Lottie (JSON)** | Smooth animation, small file | Extra dependency |

### 4.4 Animation Ideas

1. **Idle**: Gentle breathing, occasional pipe puff (smoke wisps)
2. **Thinking**: Glasses glint, hand to chin
3. **Searching**: Magnifying glass motion, eyes scan
4. **Alert**: Surprised expression, quick movement
5. **Typing**: Hands on keyboard, focused

### 4.5 Files to Replace

| Current | New |
|---------|-----|
| `public/Ship.png` | `public/Asimov-Ship.png` (space/starship theme) |
| `public/Rustle.png` | `public/Professor.png` (glasses + pipe) |
| `public/Pirate-Rustle.png` | Remove or archive |

---

## 5. Implementation Roadmap

### Phase 1: Foundation (Week 1-2)
- [ ] Execute rename script (asimov → asimov)
- [ ] Create mascot assets (Professor PNG + animation)
- [ ] Create `asimov-search` crate scaffold
- [ ] Implement metadata index (SQLite)
- [ ] Basic CLI: `asimov search` with literal matching

### Phase 2: Semantic Layer (Week 3-4)
- [ ] ONNX runtime integration
- [ ] Embedding pipeline (batch processing)
- [ ] USearch vector index integration
- [ ] Hybrid scoring (BM25 + semantic)
- [ ] Query parser with filters

### Phase 3: Performance (Week 5-6)
- [ ] Parallel indexing (Tokio)
- [ ] File watcher integration
- [ ] LRU caching layer
- [ ] Incremental update logic
- [ ] Benchmarking & optimization

### Phase 4: Proactive Engine (Week 7-8)
- [ ] `asimov-context` crate scaffold
- [ ] Window monitor
- [ ] Pattern learning
- [ ] Context prediction integration

### Phase 5: Polish (Week 9+)
- [ ] TUI integration for search results
- [ ] Keyboard shortcuts
- [ ] Privacy controls UI
- [ ] Documentation

---

## 6. Dependencies to Add

```toml
# In workspace Cargo.toml (new deps)

# Search crate
usearch = "0.11"          # Vector search
ort = "1.16"               # ONNX runtime (pure Rust)
tantivy = "0.22"           # Optional: FTS fallback
notify = "6.1"             # File watching
pdf-extract = "0.7"        # PDF text extraction
docx-rs = "0.4"            # DOCX extraction
regex = "1.10"             # Already there
lz4 = "1.24"               # Fast compression for index

# Context crate
arboard = "3.3"            # Cross-platform clipboard
xdotool = "0.1"            # Linux window control (optional)
```

---

## 7. Configuration

```json
// ~/.asimov/config.json
{
  "search": {
    "enabled": true,
    "index_path": "~/.asimov/search",
    "watched_paths": ["~/", "/home/*/"],
    "excluded_patterns": ["**/.git/**", "**/node_modules/**", "**/target/**"],
    "embedder": "miniLM",
    "batch_size": 128,
    "max_file_size_mb": 50
  },
  "context": {
    "enabled": true,
    "privacy": {
      "window_tracking": true,
      "file_tracking": true,
      "clipboard_history": true,
      "exclude_apps": ["1password", "bitwarden", "keepassxc"]
    },
    "learning": {
      "enabled": true,
      "min_occurrences": 3
    }
  },
  "branding": {
    "mascot": "professor",
    "animation": true
  }
}
```

---

## 8. Success Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Search latency (cached) | N/A | <5ms |
| Search latency (semantic) | N/A | <50ms |
| Index speed | N/A | 100k files/minute |
| Memory usage (1M files) | N/A | <500MB |
| Context prediction accuracy | N/A | >70% top-3 |

---

*Last updated: 2026-05-11*
