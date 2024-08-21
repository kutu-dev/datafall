// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{io::Cursor, path::PathBuf};

use tokio::{
    fs::{self, File},
    io,
};

use reqwest::{header::RANGE, Client, StatusCode, Url};

use std::hash::{DefaultHasher, Hash, Hasher};

pub async fn download_fragment(
    client: &Client,
    url: Url,
    mut file: File,
    range: Option<(u64, u64)>,
) -> Result<(), String> {
    let mut request = client.get(url);

    if let Some(range) = range {
        request = request.header(RANGE, format!("bytes={}-{}", range.0, range.1));
    }

    let response = request
        .send()
        .await
        .map_err(|error| format!("Downloading the file content failed: {error}"))?;

    if range.is_some() && response.status() != StatusCode::PARTIAL_CONTENT {
        return Err(String::from("The status code wasn't 206 Partial Content!"));
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
