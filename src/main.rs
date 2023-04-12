use clap::Parser;
use std::result::Result;
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Parser)]
#[clap(name = "cool-cli-app", version = "0.0.1", author = "Tyler Ross")]
struct Args{
    #[clap(short, long)]
    request: String,
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = Client::new();
    let response = client
        .get(format!("https://official-joke-api.appspot.com/jokes/{}", args.request))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{}", response.get("setup").unwrap());
    println!("{}", response.get("punchline").unwrap());
    
    Ok(())
}
