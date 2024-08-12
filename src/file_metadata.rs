use reqwest::{
    Client,
    header::{
        ACCEPT_RANGES,
        CONTENT_LENGTH,
    },
    Url,
};

#[derive(Debug)]
pub struct FileMetadata {
    pub size: Option<u64>,
    pub accept_ranges: bool,
    pub filename: String
}

impl FileMetadata {
    pub async fn new(client: &Client, url: Url) -> Result<FileMetadata, String> {
        let response = client
            .head(url.clone())
            .send()
            .await
            .map_err(|error| format!("Getting the headers for '{url}' failed: {error}"))?;

        let headers = response.headers();

        let size = match headers.get(CONTENT_LENGTH) {
            Some(header) => Some(
                header
                    .to_str().map_err(|error| format!("Non visible characters found: {error}"))?
                    .parse::<u64>().map_err(|error| format!("Invalid 'Content-Length' header: {error}"))?
            ),
            None => None,
        };

        let accept_ranges = match headers.get(ACCEPT_RANGES) {
            Some(value) => {
                let value = value
                    .to_str().map_err(|error| format!("Non visible characters found: {error}"))?;

                value == "bytes"
            },
            None => false,
        };

        let filename = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("TODO.bin")
            .to_owned();

        Ok(FileMetadata {
            size,
            accept_ranges,
            filename,
        })
    }
}
