mod file_metadata;
mod download_fragment;
mod split_download;
mod chunk;

use reqwest::{
    Client,
    Url,
};

use tokio::fs::File;

use crate::file_metadata::FileMetadata;

pub async fn download_file(url: &str) -> Result<(), String> {
    let client = Client::new();

    let url = Url
        ::parse(url)
        .map_err(|error| format!("The URL '{url}' is invalid: {error}"))?;

    let file_metadata = FileMetadata::new(
        &client,
        url.clone(),
    ).await?;

    let file = File
        ::create(&file_metadata.filename)
        .await
        .map_err(|error| format!("Couldn't create a new empty file: {error}"))?;

    if let Some(size) = file_metadata.size {
        if file_metadata.accept_ranges {
            println!("Resource '{url}' eligible for split download");
            split_download::split_download(&client, url.clone(), file, size).await?;

            return Ok(());
        }
    }

    println!("Resource '{url}' not eligible for split download");
    download_fragment::download_fragment(&client, url.clone(), file, None).await?;
    println!("Resouce '{url}' downloaded");

    Ok(())
}
