#![feature(async_await, await_macro, futures_api)]

use std::{env, process};

use futures::future::{FutureExt, TryFutureExt};

use url::Url;
use rignalrar::HubConnection;

pub fn main() {
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

        // async/await in Rust uses "v0.3" Futures. Tokio uses "v0.1" Futures
        // Fortunately there's an adaptor while the world stabilizes
        let v1future = run_hub(url)
            .unit_error()
            .boxed()
            .compat();

        tokio::run(v1future);
    } else {
        eprintln!("Usage: sample_client <URL>");
        process::exit(1);
    }
}

async fn run_hub(url: Url) {
    let mut connection = HubConnection::new(url);
    await!(connection.start()).unwrap();
}
