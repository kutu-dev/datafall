use std::{
    path::PathBuf,
};

use reqwest::{
    Client,
    Url,
};

use tokio::fs::{
    self,
    File
};

use crate::download_fragment;

pub async fn get_chunk_path(file_hash: u64, chunk_num: u64) -> Result<PathBuf, String> {
    let mut chunk_path = dirs
        ::cache_dir()
        .ok_or("{file_hash}: Chunk {chunk_num}: Unable to get the cache directory")?;

    chunk_path.push("datafall");
    
    if !chunk_path.is_dir() {
        fs::create_dir_all(&chunk_path)
            .await
            .map_err(|error| format!("{file_hash}: Chunk {chunk_num}: Failed to create the cache dir: {error}"))?;
    }

    chunk_path.push(format!("{file_hash}.part{chunk_num}"));

    Ok(chunk_path)
}

pub async fn download_chunk(client: &Client, url: Url, file_hash: u64, chunk_num: u64, chunk_size: u64, num_of_chunks: u64, file_size: u64) -> Result<(), String> {
    assert!(chunk_num < num_of_chunks);

    println!("{file_hash}: Chunk {chunk_num}: Downloading...");

    let chunk_path = get_chunk_path(file_hash, chunk_num).await?;

    if chunk_path.is_file() {
        println!("{file_hash}: Chunk {chunk_num}: Skipping!");
        return Ok(());
    }

    let fragment_file = File
        ::create(chunk_path)
        .await
        .map_err(|error| format!("{file_hash}: Chunk {chunk_num}:Couldn't create a new empty file: {error}"))?;

    let start = chunk_size * chunk_num;
    let mut end = (chunk_size * (chunk_num + 1)) - 1;
        
    // If it is the last chunk then some extra bytes must be added
    // or some data will be lost due to the round down
    // of the chunk size calculation
    if chunk_num == num_of_chunks - 1 {
        end += file_size - chunk_size * 16 + 1;
    }
        
    download_fragment::download_fragment(&client, url, fragment_file, Some((start, end))).await?;

    println!("{file_hash}: Chunk {chunk_num} download finished");

    Ok(())
}
