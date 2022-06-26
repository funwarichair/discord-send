use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, value_parser, env = "DISCORD_SEND_TOKEN")]
    token: String,
    #[clap(short, long, value_parser, env = "DISCORD_SEND_CHANNEL_ID")]
    channel_id: u64,
    #[clap(value_parser)]
    files: Vec<PathBuf>,
}

async fn run(args: Args) -> anyhow::Result<()> {
    let client = serenity::Client::builder(&args.token, Default::default())
        .await
        .context("unable to create client")?;
    let channel = serenity::model::id::ChannelId(args.channel_id);
    channel
        .send_files(&client.cache_and_http.http, &args.files, |m| m)
        .await
        .context("unable to send files")?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(err) = run(args).await {
        eprintln!("error: {:?}", err);
    }
}
