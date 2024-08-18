use std::{
    path::PathBuf,
    io::Cursor,
};

use tokio::{
    fs::{self, File},
    io,
};

use reqwest::{
    Client,
    header::{
        RANGE,
    },
    Url,
    StatusCode,
};

use std::hash::{DefaultHasher, Hash, Hasher};

pub async fn download_fragment(client: &Client, url: Url, mut file: File, range: Option<(u64, u64)>) -> Result<(), String> {
    let mut request = client.get(url);

    if let Some(range) = range {
        request = request.header(RANGE, format!("bytes={}-{}", range.0, range.1));
    }

    let response = request
        .send()
        .await
        .map_err(|error| format!("Downloading the file content failed: {error}"))?;

    if range.is_some() && response.status() != StatusCode::PARTIAL_CONTENT {
        return Err(String::from("The status code wasn't partial content!"))
    }

     let file_data = response
        .bytes()
        .await
        .map_err(|error| format!("Getting the response body as bytes failed: {error}"))?;

    let mut file_data = Cursor::new(file_data);

    io::copy(&mut file_data, &mut file)
        .await
        .map_err(|error| format!("Copying the file data failed: {error}"))?;

    Ok(())
}

