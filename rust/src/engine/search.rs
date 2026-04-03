use crate::api::types::{IndexStats, SearchField, SearchHit};
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::snippet::SnippetGenerator;
use tantivy::{Index, IndexReader, IndexWriter, ReloadPolicy};

/// Manages Tantivy search indexes for IronVault tables.
pub(crate) struct SearchEngine {
    base_dir: PathBuf,
    indexes: Mutex<HashMap<String, TableIndex>>,
}

struct TableIndex {
    index: Index,
    reader: IndexReader,
    writer: Mutex<IndexWriter>,
    id_field: Field,
    text_fields: Vec<(String, Field, f64)>, // (name, field, weight)
    #[allow(dead_code)]
    schema: Schema,
}

impl SearchEngine {
    pub(crate) fn new(db_path: &str) -> Self {
        let base = Path::new(db_path)
            .parent()
            .unwrap_or(Path::new("."))
            .join("ironvault_fts");
        Self {
            base_dir: base,
            indexes: Mutex::new(HashMap::new()),
        }
    }

    /// Build or open a search index for a table.
    pub(crate) fn build_index(&self, table: &str, fields: &[SearchField]) -> Result<()> {
        if fields.is_empty() {
            return Err(anyhow!("SearchException: at least one field required"));
        }

        let index_dir = self.base_dir.join(table);
        std::fs::create_dir_all(&index_dir)
            .with_context(|| format!("SearchException: cannot create index dir {:?}", index_dir))?;

        // Build schema
        let mut schema_builder = Schema::builder();
        let id_field = schema_builder.add_text_field("_id", STRING | STORED);

        let mut text_fields = Vec::new();
        for f in fields {
            let indexing = TextFieldIndexing::default()
                .set_tokenizer("default")
                .set_index_option(tantivy::schema::IndexRecordOption::WithFreqsAndPositions);
            let opts = if f.stored {
                TextOptions::default()
                    .set_indexing_options(indexing)
                    .set_stored()
            } else {
                TextOptions::default().set_indexing_options(indexing)
            };
            let field = schema_builder.add_text_field(&f.name, opts);
            text_fields.push((f.name.clone(), field, f.weight));
        }

        let schema = schema_builder.build();
        let index = Index::create_in_dir(&index_dir, schema.clone())
            .or_else(|_| Index::open_in_dir(&index_dir))
            .with_context(|| format!("SearchException: cannot open index for {}", table))?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .context("SearchException: cannot create reader")?;

        let writer = index
            .writer(50_000_000) // 50MB heap
            .context("SearchException: cannot create writer")?;

        let mut indexes = self.indexes.lock().unwrap();
        indexes.insert(
            table.to_string(),
            TableIndex {
                index,
                reader,
                writer: Mutex::new(writer),
                id_field,
                text_fields,
                schema,
            },
        );

        Ok(())
    }

    /// Index a single row.
    pub(crate) fn index_row(
        &self,
        table: &str,
        id: &str,
        fields: &HashMap<String, String>,
    ) -> Result<()> {
        let indexes = self.indexes.lock().unwrap();
        let ti = indexes
            .get(table)
            .ok_or_else(|| anyhow!("SearchException: no index for table '{}'", table))?;

        // Remove existing doc with this id (for re-indexing)
        let term = tantivy::Term::from_field_text(ti.id_field, id);
        let mut writer = ti.writer.lock().unwrap();
        writer.delete_term(term);

        // Build document
        let mut doc = tantivy::TantivyDocument::new();
        doc.add_text(ti.id_field, id);
        for (name, field, _weight) in &ti.text_fields {
            if let Some(text) = fields.get(name) {
                doc.add_text(*field, text);
            }
        }
        writer.add_document(doc)?;
        writer.commit().context("SearchException: commit failed")?;

        // Reload reader
        ti.reader.reload()?;

        Ok(())
    }

    /// Remove a row from the index.
    pub(crate) fn remove_from_index(&self, table: &str, id: &str) -> Result<()> {
        let indexes = self.indexes.lock().unwrap();
        let ti = indexes
            .get(table)
            .ok_or_else(|| anyhow!("SearchException: no index for table '{}'", table))?;

        let term = tantivy::Term::from_field_text(ti.id_field, id);
        let mut writer = ti.writer.lock().unwrap();
        writer.delete_term(term);
        writer.commit()?;
        ti.reader.reload()?;
        Ok(())
    }

    /// Search the index.
    pub(crate) fn search(
        &self,
        table: &str,
        query_str: &str,
        limit: u32,
        highlight: bool,
    ) -> Result<Vec<SearchHit>> {
        let indexes = self.indexes.lock().unwrap();
        let ti = indexes
            .get(table)
            .ok_or_else(|| anyhow!("SearchException: no index for table '{}'", table))?;

        let searcher = ti.reader.searcher();

        // Build query parser with all text fields
        let field_refs: Vec<Field> = ti.text_fields.iter().map(|(_, f, _)| *f).collect();
        let query_parser = QueryParser::for_index(&ti.index, field_refs.clone());
        let query = query_parser
            .parse_query(query_str)
            .map_err(|e| anyhow!("SearchException: invalid query '{}': {}", query_str, e))?;

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit as usize))
            .context("SearchException: search failed")?;

        let mut hits = Vec::with_capacity(top_docs.len());
        for (score, doc_address) in top_docs {
            let doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;

            // Extract id
            let id = doc
                .get_first(ti.id_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Generate snippet
            let snippet = if highlight && !field_refs.is_empty() {
                let mut snippet_gen = SnippetGenerator::create(&searcher, &query, field_refs[0])?;
                snippet_gen.set_max_num_chars(200);
                let s = snippet_gen.snippet_from_doc(&doc);
                s.to_html()
            } else {
                String::new()
            };

            hits.push(SearchHit {
                id,
                table: table.to_string(),
                score: score as f64,
                snippet,
            });
        }

        Ok(hits)
    }

    /// Get index statistics.
    pub(crate) fn index_stats(&self, table: &str) -> Result<IndexStats> {
        let indexes = self.indexes.lock().unwrap();
        let ti = indexes
            .get(table)
            .ok_or_else(|| anyhow!("SearchException: no index for table '{}'", table))?;

        let searcher = ti.reader.searcher();
        let num_docs = searcher.num_docs();
        let num_segments = searcher.segment_readers().len() as u64;

        // Calculate directory size
        let index_dir = self.base_dir.join(table);
        let size_bytes = dir_size(&index_dir);

        Ok(IndexStats {
            num_docs,
            num_segments,
            size_bytes,
        })
    }

    /// Check if a table has a search index and return its indexed field names.
    pub(crate) fn indexed_fields(&self, table: &str) -> Option<Vec<String>> {
        let indexes = self.indexes.lock().unwrap();
        indexes
            .get(table)
            .map(|ti| ti.text_fields.iter().map(|(name, _, _)| name.clone()).collect())
    }
}

fn dir_size(path: &Path) -> u64 {
    std::fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
                .sum()
        })
        .unwrap_or(0)
}
