use crate::config::Config;
use crate::supabase::list_videos;
use std::fs;
use std::path::Path;
use tantivy::schema::{Schema, STORED, STRING, TEXT};
use tantivy::{doc, Index, IndexWriter};

// this script rebuilds the tantivy index from the supabase videos table
const INDEX_DIR: &str = "data/index";

pub async fn rebuild(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let videos = list_videos(config).await?;
    println!("fetched {} videos from Supabase", videos.len());

    if Path::new(INDEX_DIR).exists() {
        fs::remove_dir_all(INDEX_DIR)?;
    }
    fs::create_dir_all(INDEX_DIR)?;

    let schema = build_schema();
    let index = Index::create_in_dir(INDEX_DIR, schema.clone())?;
    let mut writer: IndexWriter = index.writer(50_000_000)?;

    let f_id = schema.get_field("id").unwrap();
    let f_title = schema.get_field("title").unwrap();
    let f_description = schema.get_field("description").unwrap();
    let f_channel = schema.get_field("channel_title").unwrap();
    let f_language = schema.get_field("language").unwrap();
    let f_url = schema.get_field("url").unwrap();
    let f_thumb = schema.get_field("thumbnail_url").unwrap();
    let f_views = schema.get_field("view_count").unwrap();
    let f_likes = schema.get_field("like_count").unwrap();

    for v in &videos {
        writer.add_document(doc!(
            f_id => v.id.as_str(),
            f_title => v.title.as_str(),
            f_description => v.description.as_str(),
            f_channel => v.channel_title.as_deref().unwrap_or(""),
            f_language => v.language.as_str(),
            f_url => v.url.as_str(),
            f_thumb => v.thumbnail_url.as_deref().unwrap_or(""),
            f_views => v.view_count.to_string(),
            f_likes => v.like_count.to_string(),
        ))?;
    }

    writer.commit()?;
    println!("indexed {} documents → {INDEX_DIR}", videos.len());
    Ok(())
}

fn build_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("id", STRING | STORED);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("description", TEXT);
    schema_builder.add_text_field("channel_title", STRING | STORED);
    schema_builder.add_text_field("language", STRING | STORED);
    schema_builder.add_text_field("url", STRING | STORED);
    schema_builder.add_text_field("thumbnail_url", STRING | STORED);
    schema_builder.add_text_field("view_count", STRING | STORED);
    schema_builder.add_text_field("like_count", STRING | STORED);
    schema_builder.build()
}
