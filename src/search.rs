use serde::Serialize;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value};
use tantivy::{Index, TantivyDocument};
use std::collections::HashMap;

const INDEX_DIR: &str = "data/index";
const RRF_K: f32 = 60.0;

pub fn fuse_rrf(
    bm25: &[SearchHit],
    vector: &[crate::supabase::VectorHit],
    limit: usize,
) -> Vec<SearchHit> {
    let mut scores: HashMap<String, f32> = HashMap::new();
    let mut docs: HashMap<String, SearchHit> = HashMap::new();

    for (rank, hit) in bm25.iter().enumerate() {
        *scores.entry(hit.id.clone()).or_insert(0.0) += 1.0 / (RRF_K + rank as f32 + 1.0);
        docs.entry(hit.id.clone()).or_insert_with(|| hit.clone());
    }

    for (rank, hit) in vector.iter().enumerate() {
        *scores.entry(hit.id.clone()).or_insert(0.0) += 1.0 / (RRF_K + rank as f32 + 1.0);
        docs.entry(hit.id.clone()).or_insert_with(|| SearchHit {
            id: hit.id.clone(),
            title: hit.title.clone(),
            channel_title: hit.channel_title.clone().unwrap_or_default(),
            language: hit.language.clone(),
            url: hit.url.clone(),
            thumbnail_url: hit.thumbnail_url.clone().unwrap_or_default(),
            view_count: hit.view_count,
            like_count: hit.like_count,
            score: hit.similarity as f32,
        });
    }

    let mut merged: Vec<SearchHit> = docs
        .into_iter()
        .map(|(id, mut hit)| {
            hit.score = *scores.get(&id).unwrap_or(&0.0);
            hit
        })
        .collect();

    merged.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    merged.truncate(limit);
    merged
}


#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    pub id: String,
    pub title: String,
    pub channel_title: String,
    pub language: String,
    pub url: String,
    pub thumbnail_url: String,
    pub view_count: i64,
    pub like_count: i64,
    pub score: f32,
}

pub fn search_bm25(query: &str, limit: usize) -> Result<Vec<SearchHit>, Box<dyn std::error::Error>> {
    // the lgoic here is to open the index, parse the query, and return the top hits 

    let index = Index::open_in_dir(INDEX_DIR)?;
    let schema = index.schema();
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let title = schema.get_field("title").unwrap();
    let description = schema.get_field("description").unwrap();

    let parser = QueryParser::for_index(&index, vec![title, description]);
    let query = parser.parse_query(query)?;

    let top = searcher.search(&query, &TopDocs::with_limit(limit))?;

    let mut hits = Vec::new();
    for (score, addr) in top {
        let doc: TantivyDocument = searcher.doc(addr)?;

        // searchhit is a struct that represents a search result, and we populate it with the fields from the document
        hits.push(SearchHit {
            id: text_field(&schema, &doc, "id"),
            title: text_field(&schema, &doc, "title"),
            channel_title: text_field(&schema, &doc, "channel_title"),
            language: text_field(&schema, &doc, "language"),
            url: text_field(&schema, &doc, "url"),
            thumbnail_url: text_field(&schema, &doc, "thumbnail_url"),
            view_count: text_field(&schema, &doc, "view_count").parse().unwrap_or(0),
            like_count: text_field(&schema, &doc, "like_count").parse().unwrap_or(0),
            score,
        });
    }

    Ok(hits)
}


pub async fn search_hybrid(
    config: &crate::config::Config,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchHit>, Box<dyn std::error::Error>> {
    let bm25 = search_bm25(query, limit)?;
    let emb = crate::embed::embed_text(
        &config.openrouter_api_key,
        &config.openrouter_embed_model,
        query,
    )
    .await?;
    let vector = crate::supabase::match_videos(config, &emb, limit as i64, None).await?;
    Ok(fuse_rrf(&bm25, &vector, limit))
}

fn text_field(schema: &Schema, doc: &TantivyDocument, name: &str) -> String {
    let field = schema.get_field(name).unwrap();
    doc.get_first(field)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}