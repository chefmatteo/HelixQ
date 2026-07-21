use crate::config::Config;
use crate::youtube_inject::Video;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct VectorHit {
    // retrieve the following fields from the supabase videos table
    // id, title, description, channel_title, language, view_count, like_count, thumbnail_url, url, similarity
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel_title: Option<String>, // optional
    pub language: String,
    pub view_count: i64,
    pub like_count: i64, // integer
    pub thumbnail_url: Option<String>, // optional
    pub url: String,
    pub similarity: f64, // float
}

pub async fn match_videos(
    // match the embedding to the videos table and return the top match_count videos
    config: &Config,
    embedding: &[f32],
    match_count: i64,
    filter_language: Option<&str>,
) -> Result<Vec<VectorHit>, Box<dyn std::error::Error>> {
    let url = format!(
        "{}/rest/v1/rpc/match_videos",
        config.supabase_url.trim_end_matches('/')
    );

    let body = serde_json::json!({
        "query_embedding": embedding,
        "match_count": match_count,
        "filter_language": filter_language,
    });

    let response = reqwest::Client::new()
        .post(&url)
        .header("apikey", &config.supabase_service_role_key)
        .header(
            "Authorization",
            format!("Bearer {}", config.supabase_service_role_key),
        )
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;
    if !status.is_success() {
        return Err(format!("match_videos failed ({status}): {text}").into());
    }

    Ok(serde_json::from_str(&text)?)

}
#[derive(Serialize)]
struct VideoRow<'a> {
    id: &'a str,
    title: &'a str,
    description: &'a str,
    channel_id: &'a str,
    channel_title: &'a str,
    language: &'a str,
    view_count: i64,
    like_count: i64,
    thumbnail_url: &'a str,
    url: &'a str,
    /// JSON number array — Supabase/PostgREST maps this to pgvector.
    embedding: &'a [f32],
}

#[derive(Debug, Clone, Deserialize)]
pub struct StoredVideo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel_title: Option<String>,
    pub language: String,
    pub view_count: i64,
    pub like_count: i64,
    pub thumbnail_url: Option<String>,
    pub url: String,
}

pub async fn list_videos(
    config: &Config,
) -> Result<Vec<StoredVideo>, Box<dyn std::error::Error>> {
    // get all rows from videos 
    
    let url = format!(
        "{}/rest/v1/videos?select=id,title,description,channel_title,language,view_count,like_count,thumbnail_url,url",
        config.supabase_url.trim_end_matches('/')
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header("apikey", &config.supabase_service_role_key)
        .header(
            "Authorization",
            format!("Bearer {}", config.supabase_service_role_key),
        )
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        return Err(format!("list_videos failed ({status}): {body}").into());
    }

    let videos: Vec<StoredVideo> = serde_json::from_str(&body)?;
    Ok(videos)
}

pub async fn upsert_video(
    config: &Config,
    video: &Video,
    embedding: &[f32],
) -> Result<(), Box<dyn std::error::Error>> {
    if embedding.is_empty() {
        return Err("embedding is empty — refuse to upsert".into());
    }

    println!(
        "  upsert payload: embedding_len={}, first3={:?}",
        embedding.len(),
        embedding.iter().take(3).collect::<Vec<_>>()
    );

    let url = format!(
        "{}/rest/v1/videos?on_conflict=id",
        config.supabase_url.trim_end_matches('/')
    );

    let row = VideoRow {
        id: &video.id,
        title: &video.title,
        description: &video.description,
        channel_id: &video.channel_id,
        channel_title: &video.channel_title,
        language: &video.language,
        view_count: video.view_count,
        like_count: video.like_count,
        thumbnail_url: &video.thumbnail_url,
        url: &video.url,
        embedding,
    };

    let response = reqwest::Client::new()
        .post(&url)
        .header("apikey", &config.supabase_service_role_key)
        .header(
            "Authorization",
            format!("Bearer {}", config.supabase_service_role_key),
        )
        .header("Content-Type", "application/json")
        .header("Prefer", "resolution=merge-duplicates,return=representation")
        .json(&row)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(format!("Supabase upsert failed ({status}): {body}").into());
    }

    println!("  supabase response: {body}");
    Ok(())
}


