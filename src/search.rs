use serde::Serialize;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, Value};
use tantivy::{Index, TantivyDocument};

const INDEX_DIR: &str = "data/index";

#[derive(Debug, Serialize)]
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

fn text_field(schema: &Schema, doc: &TantivyDocument, name: &str) -> String {
    let field = schema.get_field(name).unwrap();
    doc.get_first(field)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}