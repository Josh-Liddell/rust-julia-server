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
    Example,
    RustPrice,
    JuliaAdd {
        #[arg(short)]
        a: f64,
        #[arg(short)]
        b: f64,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Example => dialog::example_dialog(),
        Commands::RustPrice => dialog::price_option_rust(),
        Commands::JuliaAdd { a, b } => {
            let client = Client::new();
            let mut map = HashMap::new();
            map.insert("a", a);
            map.insert("b", b);

            // let task = AdditionTask { a, b };

            let response = client
                .post("http://localhost:8080/post_test")
                .json(&map) // serializes to JSON
                .send()
                .await
                .expect("request failed");

            let result = response.text().await.expect("cannot read response");

            println!("Result: {result}");
        }
    }
}

// figure out how to use the one defined in the other crate....
// #[derive(Serialize, Deserialize, Debug)]
// struct AdditionTask {
//     a: f64,
//     b: f64,
// }
