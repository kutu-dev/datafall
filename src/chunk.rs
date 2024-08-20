use std::path::PathBuf;
use tokio::fs::{self, File};

use reqwest::{Client, Url};

use crate::download_fragment;

pub async fn get_chunk_path(file_hash: &str, chunk_num: u64) -> Result<PathBuf, String> {
    let mut chunk_path = dirs::cache_dir().ok_or("Unable to get the cache directory")?;

    chunk_path.push("datafall");
    chunk_path.push(file_hash);

    if !chunk_path.is_dir() {
        fs::create_dir_all(&chunk_path)
            .await
            .map_err(|error| format!("Failed to create the chunks cache dir: {error}"))?;
    }

    chunk_path.push(format!("part{chunk_num}.fragment"));

    Ok(chunk_path)
}

pub async fn download_chunk(
    client: Client,
    url: Url,
    chunk_num: u64,
    num_of_chunks: u64,
    chunk_size: u64,
    file_hash: &str,
    file_size: u64,
) -> Result<(), String> {
    assert!(chunk_num < num_of_chunks);

    let chunk_path = get_chunk_path(file_hash, chunk_num).await?;

    if chunk_path.is_file() {
        return Ok(());
    }

    let fragment_file = File::create(chunk_path)
        .await
        .map_err(|error| format!("Couldn't create a new empty file: {error}"))?;

    let start = chunk_size * chunk_num;
    let mut end = (chunk_size * (chunk_num + 1)) - 1;

    // If it is the last chunk then some extra bytes must be added
    // or some data will be lost due to the round down
    // of the chunk size calculation
    if chunk_num == num_of_chunks - 1 {
        end += file_size - chunk_size * 16 + 1;
    }

    download_fragment::download_fragment(&client, url, fragment_file, Some((start, end))).await?;

    Ok(())
}
