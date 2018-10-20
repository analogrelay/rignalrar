use std::io::{self, Write};

use url::Url;

use hyper::{Body, Client, Request};
use hyper::rt::{Future, Stream};
use hyper::client::HttpConnector;

// Bring in the .compat() extension so we can await! hyper's futures
use futures::compat::Future01CompatExt;

use crate::error::Error;

/** Represents a connection to a SignalR Hub */
pub struct HubConnection {
    url: Url,
    client: Client<HttpConnector>,
}

impl HubConnection {
    pub fn new<U: Into<Url>>(url: U) -> HubConnection {
        HubConnection {
            url: url.into(),
            client: Client::builder().build_http(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        // Build the negotiate URL
        let negotiate_url = {
            let mut url = self.url.clone();
            url.path_segments_mut().unwrap().push("negotiate");
            url
        };
        println!("negotiating with {}", negotiate_url);

        // Negotiate
        let req = Request::post(negotiate_url.as_str())
            .header("Content-Length", 0)
            .body(Body::empty())
            .unwrap();
        let response = await!(self.client.request(req).compat())?;
        let content = await!(response.into_body().concat2().compat())?;
        io::stdout().write_all(&content)?;
        Ok(())
    }
}