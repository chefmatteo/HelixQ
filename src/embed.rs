use serde::Deserialize;
use serde_json::json;

/// Must match `videos.embedding vector(1586)` in Supabase.
pub const EMBED_DIMS: usize = 1586;

#[derive(Debug, Deserialize)]
struct EmbedResponse {
    data: Vec<EmbedItem>,
}

#[derive(Debug, Deserialize)]
struct EmbedItem {
    embedding: Vec<f32>,
}

pub async fn embed_text(
    api_key: &str,
    model: &str,
    text: &str,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let body = json!({
        "model": model,
        "input": text,
        "dimensions": EMBED_DIMS
    });

    let response = reqwest::Client::new()
        .post("https://openrouter.ai/api/v1/embeddings")
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let raw = response.text().await?;
    if !status.is_success() {
        return Err(format!("OpenRouter embed failed ({status}): {raw}").into());
    }

    let parsed: EmbedResponse = serde_json::from_str(&raw)?;
    let embedding = parsed
        .data
        .into_iter()
        .next()
        .map(|item| item.embedding)
        .ok_or("OpenRouter returned no embedding")?;

    if embedding.len() != EMBED_DIMS {
        return Err(format!("expected {EMBED_DIMS} dims, got {}", embedding.len()).into());
    }

    Ok(embedding)
}
