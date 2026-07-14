mod config;
mod youtube_inject;

use config::Config;
use youtube_inject::fetch_video;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Load config (fails early if .env keys are missing)
    let config = Config::from_env().map_err(|e| {
        eprintln!("config error: {e}");
        e
    })?;
    config.print_status();

    // 2) Fetch one video (change this id anytime)
    let video_id = "jNQXAC9IVRw";
    println!("\nFetching YouTube video: {video_id}");

    let video = fetch_video(&config.youtube_api_key, video_id).await?;

    println!("id:      {}", video.id);
    println!("title:   {}", video.title);
    println!("channel: {}", video.channel_title);
    println!("lang:    {}", video.language);
    println!("views:   {}", video.view_count);
    println!("likes:   {}", video.like_count);
    println!("thumb:   {}", video.thumbnail_url);
    println!("url:     {}", video.url);
    println!("desc:    {}", video.description.chars().take(120).collect::<String>());

    Ok(())
}
