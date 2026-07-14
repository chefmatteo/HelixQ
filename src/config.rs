use std::env;

/// Secrets and config loaded from `.env` / the process environment.
pub struct Config {
    pub supabase_url: String,
    pub supabase_service_role_key: String,
    pub youtube_api_key: String,
}

// impl config is taking the values from the .env file and storing them in the struct 
impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        Ok(Self {
            supabase_url: required("SUPABASE_URL")?,
            supabase_service_role_key: required("SUPABASE_SERVICE_ROLE_KEY")?,
            youtube_api_key: required("YOUTUBE_API_KEY")?,
        })
    }

    /// Prints whether each key is present — never prints secret values.
    pub fn print_status(&self) {
        println!("HelixQ ingest | env check");
        println!("  SUPABASE_URL: ok ({} chars)", self.supabase_url.len());
        println!(
            "  SUPABASE_SERVICE_ROLE_KEY: ok ({} chars)",
            self.supabase_service_role_key.len()
        );
        println!("  YOUTUBE_API_KEY: ok ({} chars)", self.youtube_api_key.len());
    }
}

fn required(key: &str) -> Result<String, String> {
    match env::var(key) {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(format!("{key} is missing or empty in .env")),
    }
}
