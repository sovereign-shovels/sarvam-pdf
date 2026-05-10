use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod config;
mod pdf;

use config::Config;

#[derive(Parser)]
#[command(name = "sarvam-pdf")]
#[command(about = "Drag a PDF, get it in your language. 22 Indic languages.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract text from a PDF
    Extract {
        /// Path to PDF file
        path: PathBuf,
    },
    /// Translate text using Sarvam API
    Translate {
        /// Text to translate
        text: String,
        /// Source language code
        #[arg(short, long, default_value = "en-IN")]
        from: String,
        /// Target language code
        #[arg(short, long, default_value = "hi-IN")]
        to: String,
    },
    /// Extract text from PDF and translate it
    Convert {
        /// Path to PDF file
        path: PathBuf,
        /// Source language code
        #[arg(short, long, default_value = "en-IN")]
        from: String,
        /// Target language code
        #[arg(short, long, default_value = "hi-IN")]
        to: String,
        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Serialize)]
struct TranslateRequest {
    source_language_code: String,
    target_language_code: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TranslateResponse {
    translated_text: String,
}

async fn translate(cfg: &Config, text: &str, from: &str, to: &str) -> Result<String> {
    let api_key = cfg.api_key().ok_or_else(|| anyhow::anyhow!("SARVAM_API_KEY not set"))?;
    let endpoint = cfg.endpoint.clone().unwrap_or_default();

    let body = TranslateRequest {
        source_language_code: from.into(),
        target_language_code: to.into(),
        text: text.into(),
        model: cfg.model.clone(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&endpoint)
        .header("api-subscription-key", api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("API error ({}): {}", status, body_text));
    }

    let parsed: TranslateResponse = response.json().await?;
    Ok(parsed.translated_text)
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = Config::load();

    match cli.command {
        Commands::Extract { path } => {
            let text = pdf::extract_text(&path)?;
            println!("{}", text);
        }

        Commands::Translate { text, from, to } => {
            let result = translate(&cfg, &text, &from, &to).await?;
            println!("{}", result);
        }

        Commands::Convert { path, from, to, output } => {
            println!("Extracting text from {}...", path.display());
            let text = pdf::extract_text(&path)?;
            println!("Extracted {} characters. Translating...", text.len());

            // Translate in chunks if text is very long
            let chunk_size = 4000;
            let mut translated = String::new();

            for chunk in text.chars().collect::<Vec<_>>().chunks(chunk_size) {
                let chunk_str: String = chunk.iter().collect();
                match translate(&cfg, &chunk_str, &from, &to).await {
                    Ok(t) => translated.push_str(&t),
                    Err(e) => {
                        eprintln!("Translation error for chunk: {}", e);
                        translated.push_str(&chunk_str);
                    }
                }
            }

            if let Some(out) = output {
                tokio::fs::write(&out, translated).await?;
                println!("Saved translated text to {}", out.display());
            } else {
                println!("\n{}", translated);
            }
        }
    }

    Ok(())
}
