use std::{
    path::PathBuf,
    fs::{self, File},
    io::{self, Cursor}
};

use reqwest::{
    blocking::Client,
    header::{
        RANGE,
    },
    Url,
    StatusCode,
};

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::download_fragment;

pub fn get_chunk_path(file_hash: u64, chunk_num: u64) -> Result<PathBuf, String> {
    let mut chunk_path = dirs
        ::cache_dir()
        .ok_or("Unable to get the cache directory")?;

    chunk_path.push("datafall");
    
    if !chunk_path.is_dir() {
        fs::create_dir_all(&chunk_path)
            .map_err(|error| format!("Failed to create the cache dir: {error}"))?;
    }

    chunk_path.push(format!("{file_hash}.part{chunk_num}"));

    Ok(chunk_path)
}

pub fn download_chunk(client: &Client, url: Url, file_hash: u64, chunk_num: u64, chunk_size: u64, num_of_chunks: u64, file_size: u64) -> Result<(), String> {
    assert!(chunk_num < num_of_chunks);

    println!("Chunk: {chunk_num}");

    let chunk_path = get_chunk_path(file_hash, chunk_num)?;

    if chunk_path.is_file() {
        println!("This chunk has already been downloaded, skipping!");
        return Ok(());
    }

    let fragment_file = File
        ::create(chunk_path)
        .map_err(|error| format!("Couldn't create a new empty file: {error}"))?;

    let start = chunk_size * chunk_num;
    let mut end = (chunk_size * (chunk_num + 1)) - 1;
        
    // If it is the last chunk then some extra bytes must be added
    // or some data will be lost due to the round down
    // of the chunk size calculation
    if chunk_num == num_of_chunks - 1 {
        end += file_size - chunk_size * 16 + 1;
    }

    println!("With range: {start}-{end}");
        
    download_fragment::download_fragment(&client, url, fragment_file, Some((start, end)))?;

    Ok(())
}
