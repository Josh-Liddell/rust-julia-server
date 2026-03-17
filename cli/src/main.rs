mod dialog;
mod price;

use clap::{Parser, Subcommand};
use reqwest::Client;
use std::collections::HashMap;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(long)]
    // one: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Example of dialoguer crate feature")]
    Example,
    #[command(about = "Price a european option in rust")]
    RustPrice,
    #[command(about = "Stop the running server")]
    Stop,
    #[command(about = "Add two numbers using the Julia server")]
    JuliaAdd {
        #[arg(short)]
        a: f64,
        #[arg(short)]
        b: f64,
    },
    Number,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Example => dialog::example_dialog(),
        Commands::RustPrice => dialog::price_option_rust(),
        Commands::Stop => {
            let client = Client::new();
            client
                .post("http://localhost:8080/stop/true")
                .send()
                .await
                .expect("request failed");
        }
        Commands::JuliaAdd { a, b } => {
            let client = Client::new();
            let mut map = HashMap::new();
            map.insert("a", a);
            map.insert("b", b);

            let response = client
                .post("http://localhost:8080/post_test")
                .json(&map) // serializes to JSON
                .send()
                .await
                .expect("request failed");

            let result = response.text().await.expect("cannot read response");

            println!("Result: {result}");
        }
        Commands::Number => {
            let body = reqwest::get("http://localhost:8080/number")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();

            println!("number = {body}");
        }
    }
}
