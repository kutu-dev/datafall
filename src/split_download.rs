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

use crate::chunk;

pub fn split_download(client: &Client, url: Url, mut final_file: File, file_size: u64) -> Result<(), String> {
    let mut hasher = DefaultHasher::new();
    url.as_str().hash(&mut hasher);
    file_size.hash(&mut hasher);
    let file_hash = hasher.finish();

    println!("File hash: {file_hash}");

    const NUM_OF_CHUNKS: u64 = 16;

    let chunk_size = file_size / NUM_OF_CHUNKS;
    println!("Chunk size: {chunk_size} bytes");

    for chunk_num in 0..NUM_OF_CHUNKS {
        chunk::download_chunk(&client, url.clone(), file_hash, chunk_num, chunk_size, NUM_OF_CHUNKS, file_size)?;
    }

    println!("Merging downloaded chunks...");
    for chunk_num in 0..NUM_OF_CHUNKS {
        println!("Merging chunk {chunk_num}");

        let chunk_path = chunk::get_chunk_path(file_hash, chunk_num)?;

        let mut chunk_file = File
            ::open(&chunk_path)
            .map_err(|error| format!("The chunk file is missing!: {error}"))?;

        io::copy(&mut chunk_file, &mut final_file)
            .map_err(|error| format!("Copying the chunk file data to the final file failed: {error}"))?;

        fs::remove_file(&chunk_path)
         .map_err(|error| format!("Failed to remove the chunk file: {error}"))?;
    }

    Ok(())
}

