use std::{env, process};

use url::Url;
use rignalrar::HubConnection;

#[tokio::main]
pub async fn main() {
    println!("SignalR Sample Client");

    // Argument parsing
    let mut url_argument = None;
    for argument in env::args().skip(1) {
        if url_argument.is_none() {
            url_argument = Some(argument);
        }
    }

    if let Some(url) = url_argument {
        let url = url.parse::<Url>().expect("Invalid URL");

        run_hub(url).await;
    } else {
        eprintln!("Usage: sample_client <URL>");
        process::exit(1);
    }
}

async fn run_hub(url: Url) {
    let mut connection = HubConnection::new(url);
    connection.start().await.unwrap();
}
