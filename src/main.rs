mod config;
mod embed;
mod supabase;
mod youtube_inject;
mod index; 
mod search; 

use clap::{Parser, Subcommand};
use config::Config;
use embed::embed_text;
use supabase::upsert_video;
use youtube_inject::fetch_video;

#[derive(Parser)]
#[command(name = "HelixQ", about = "HelixQ YouTube ingest CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch videos by id, embed title+description, upsert into Supabase
    Fetch {
        /// Comma-separated YouTube video ids
        #[arg(long)]
        ids: String,
    },

    /// Rebuild Tantivy index from Supabase videos
    Index,

    /// BM25 keyword search against the local Tantivy index
    Search {
        #[arg(long)]
        q: String,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = Config::from_env().map_err(|e| {
        eprintln!("config error: {e}");
        e
    })?;
    config.print_status();

    match cli.command {
        Commands::Fetch { ids } => {
            let video_ids = parse_ids(&ids);
            if video_ids.is_empty() {
                return Err("pass at least one id: --ids id1,id2".into());
            }

            println!("\nIngesting {} video(s)...", video_ids.len());
            let mut ok = 0usize;
            let mut failed = 0usize;

            for video_id in &video_ids {
                match ingest_one(&config, video_id).await {
                    Ok(()) => ok += 1,
                    Err(err) => {
                        failed += 1;
                        eprintln!("failed {video_id}: {err}");
                    }
                }
            }

            println!("\nDone. saved={ok} failed={failed}");
        }

        Commands::Index => {
            index::rebuild(&config).await?; //rebuild: rebuild the index in the data/index directory
        }

        Commands::Search { q, limit } => {
            let hits = search::search_hybrid(&config, &q, limit).await?;
            println!("\n{} hybrid hit(s) for {:?}", hits.len(), q);
            for (i, hit) in hits.iter().enumerate() {
                println!(
                    "{}. [{:.4}] {} — {} ({})",
                    i + 1,
                    hit.score,
                    hit.title,
                    hit.channel_title,
                    hit.id
                );
            }
        }
    }

    Ok(())
}

fn parse_ids(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .collect()
}

async fn ingest_one(config: &Config, video_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n--- {video_id} ---");
    println!("Fetching...");
    let video = fetch_video(&config.youtube_api_key, video_id).await?;

    let text = format!("{}\n{}", video.title, video.description);
    println!("Embedding...");
    let embedding = embed_text(
        &config.openrouter_api_key,
        &config.openrouter_embed_model,
        &text,
    )
    .await?;
    println!("embedding dims: {}", embedding.len());

    println!("Upserting...");
    upsert_video(config, &video, &embedding).await?;
    println!("saved: {} | {}", video.id, video.title);

    Ok(())
}
