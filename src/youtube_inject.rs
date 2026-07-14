use serde::Deserialize;


// private structs (youtube responses stays inside youtube_inject.rs, only video and fetch_video are being exported)
#[derive(Debug, Deserialize)]
struct YoutubeResponse {
    items: Vec<YoutubeItem>,
}

#[derive(Debug, Deserialize)]
struct YoutubeItem {
    id: String,
    snippet: Snippet,
    statistics: Statistics,
}

#[derive(Debug, Deserialize)]
struct Snippet {
    title: String,
    description: String,
    #[serde(rename = "channelId")]
    channel_id: String,
    #[serde(rename = "channelTitle")]
    channel_title: String,
    #[serde(default)]
    #[serde(rename = "defaultLanguage")]
    default_language: Option<String>,
    #[serde(default)]
    #[serde(rename = "defaultAudioLanguage")]
    default_audio_language: Option<String>,
    thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize)]
struct Thumbnails {
    #[serde(default)]
    high: Option<Thumbnail>,
    #[serde(default)]
    medium: Option<Thumbnail>,
    #[serde(default)]
    default: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
struct Thumbnail {
    url: String,
}

#[derive(Debug, Deserialize)]
struct Statistics {
    #[serde(rename = "viewCount")]
    view_count: String,
    #[serde(rename = "likeCount", default)]
    like_count: Option<String>,
}

/// Clean video shape we care about inside HelixQ (matches Supabase columns later).
#[derive(Debug, Clone)] //clone: create a new instance of the struct 
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: String,
    pub channel_id: String,
    pub channel_title: String,
    pub language: String,
    pub view_count: i64,
    pub like_count: i64,
    pub thumbnail_url: String,
    pub url: String,
}

pub async fn fetch_video(
    api_key: &str,
    video_id: &str,
) -> Result<Video, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet,statistics&id={video_id}&key={api_key}"
    );

    let response: YoutubeResponse = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let item = response
        .items
        .into_iter()
        .next()
        .ok_or_else(|| format!("No video found for id={video_id}"))?;

    Ok(item.into_video())
}

impl YoutubeItem {
    fn into_video(self) -> Video {
        let thumb = self
            .snippet
            .thumbnails
            .high
            .or(self.snippet.thumbnails.medium)
            .or(self.snippet.thumbnails.default)
            .map(|t| t.url)
            .unwrap_or_default();

        let language = self
            .snippet
            .default_language
            .or(self.snippet.default_audio_language)
            .unwrap_or_else(|| "und".to_string());

        let view_count = self.statistics.view_count.parse().unwrap_or(0);
        let like_count = self
            .statistics
            .like_count
            .unwrap_or_else(|| "0".into())
            .parse()
            .unwrap_or(0);

        // Keep descriptions short for storage + embeddings later.
        let description: String = self.snippet.description.chars().take(800).collect();

        Video {
            id: self.id.clone(),
            title: self.snippet.title,
            description,
            channel_id: self.snippet.channel_id,
            channel_title: self.snippet.channel_title,
            language,
            view_count,
            like_count,
            thumbnail_url: thumb,
            url: format!("https://www.youtube.com/watch?v={}", self.id),
        }
    }
}
