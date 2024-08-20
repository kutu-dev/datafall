use reqwest::{
    header::{ACCEPT_RANGES, CONTENT_LENGTH},
    Client, Url,
};

#[derive(Debug)]
pub struct ResourceMetadata {
    pub size: Option<u64>,
    pub accept_ranges: bool,
}

impl ResourceMetadata {
    pub async fn new(client: Client, url: Url) -> Result<ResourceMetadata, String> {
        let response = client
            .head(url.as_str())
            .send()
            .await
            .map_err(|error| format!("Getting the headers for '{url}' failed: {error}"))?;

        let headers = response.headers();

        let size = match headers.get(CONTENT_LENGTH) {
            Some(header) => Some(
                header
                    .to_str()
                    .map_err(|error| format!("Non visible characters found: {error}"))?
                    .parse::<u64>()
                    .map_err(|error| format!("Invalid 'Content-Length' header: {error}"))?,
            ),
            None => None,
        };

        let accept_ranges = match headers.get(ACCEPT_RANGES) {
            Some(value) => {
                let value = value
                    .to_str()
                    .map_err(|error| format!("Non visible characters found: {error}"))?;

                value == "bytes"
            }
            None => false,
        };

        Ok(ResourceMetadata {
            size,
            accept_ranges,
        })
    }
}
