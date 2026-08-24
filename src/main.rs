use clap::Parser;

use anyhow::Context;
use codex_free::config::{Cli, CliCommand, load_config};
use codex_free::quickstart;
use codex_free::server::start_http_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    if let Err(error) = run().await {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let mut cli = Cli::parse();
    if let Some(command) = cli.command.take() {
        match command {
            CliCommand::Quickstart(args) => {
                let outcome = quickstart::run(args)?;
                if !outcome.start_server {
                    return Ok(());
                }
                cli.work_dir = Some(outcome.work_dir.to_string_lossy().into_owned());
                cli.config = Some(outcome.config_path.to_string_lossy().into_owned());
            }
        }
    }

    let config = load_config(cli).map_err(anyhow::Error::msg)?;
    start_http_server(config)
        .await
        .context("start Codex Free server")
}
