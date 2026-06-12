mod commands;
mod output;
mod relations;
mod summary;

use clap::{Parser, Subcommand};
use embedder::{Backend as EmbedderBackend, EmbedderBuilder};
use std::path::PathBuf;
use std::process::ExitCode;
use vectordb::{Backend as StoreBackend, VectorStoreBuilder};

#[derive(Parser)]
#[command(name = "surreal_core")]
struct Cli {
    /// Path to the embedded SurrealKV database file.
    #[arg(long, default_value = ".surreal/research.db")]
    db_path: String,

    /// Path to the directory containing per-paper subdirectories with `summary.md`.
    #[arg(long, default_value = "research")]
    research_dir: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// (Re)create schema and ingest all summaries.
    Init,
    /// Ingest only summaries not already in the store.
    Update,
    /// Run a similarity search over chunk embeddings.
    Query {
        question: String,
        #[arg(long, default_value_t = 8)]
        top_k: usize,
    },
    /// Graph traversal over cites/same_topic edges.
    Related {
        paper_id: String,
        #[arg(long, default_value_t = 1)]
        depth: u32,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(&cli).await {
        Ok(value) => output::success(&value),
        Err(err) => output::failure(&err),
    }
}

async fn run(cli: &Cli) -> anyhow::Result<serde_json::Value> {
    let store = VectorStoreBuilder::new(StoreBackend::Surreal {
        path: cli.db_path.clone(),
    })
    .build()
    .await?;

    let embedder = EmbedderBuilder::new(EmbedderBackend::FastEmbed).build()?;

    let result = match &cli.command {
        Command::Init => commands::init::run(store.as_ref(), embedder.as_ref(), &cli.research_dir).await,
        Command::Update => commands::update::run(store.as_ref(), embedder.as_ref(), &cli.research_dir).await,
        Command::Query { question, top_k } => {
            commands::query::run(store.as_ref(), embedder.as_ref(), question, *top_k).await
        }
        Command::Related { paper_id, depth } => commands::related::run(store.as_ref(), paper_id, *depth).await,
    };

    store.close().await?;
    result
}
