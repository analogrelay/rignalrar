use url::Url;

use bytes::buf::BufExt;
use hyper::{Body, Client, Request};
use hyper::client::HttpConnector;

use crate::error::Error;

#[derive(Deserialize)]
struct NegotiateResponse {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "availableTransports")]
    pub available_transports: Vec<AvailableTransport>
}

#[derive(Deserialize)]
struct AvailableTransport {
    #[serde(rename = "transport")]
    pub name: String,
    #[serde(rename = "transferFormats")]
    pub transfer_formats: Vec<String>
}

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
            // Empty body doesn't put any Content-Length in by default :(
            .header("Content-Length", 0)
            .body(Body::empty())
            .unwrap();
        let response = self.client.request(req).await?;
        let content = hyper::body::aggregate(response).await?;

        // Parse the response
        let resp: NegotiateResponse = serde_json::from_reader(content.reader())?;

        println!("connectionId: {}", resp.connection_id);
        println!("available transports:");
        for transport in resp.available_transports {
            println!("  * {} (transfer formats: {})", transport.name, transport.transfer_formats.join(", "));
        }

        Ok(())
    }
}