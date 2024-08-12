use tokio::{
    fs::{self, File},
    io,
};

use reqwest::{
    Client,
    Url,
};

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::chunk;

pub async fn split_download(client: &Client, url: Url, mut final_file: File, file_size: u64) -> Result<(), String> {
    let mut hasher = DefaultHasher::new();
    url.as_str().hash(&mut hasher);
    file_size.hash(&mut hasher);
    let file_hash = hasher.finish();

    println!("{file_hash}: Starting split download of resource '{url}'");

    const NUM_OF_CHUNKS: u64 = 16;

    let chunk_size = file_size / NUM_OF_CHUNKS;
    println!("{file_hash}: Chunk size: {chunk_size} bytes");

    let mut tasks = Vec::new();

    for chunk_num in 0..NUM_OF_CHUNKS {
        let url = url.clone();
        let client = reqwest::Client::new();

        let task = tokio::spawn(async move {
            chunk::download_chunk(&client, url, file_hash, chunk_num, chunk_size, NUM_OF_CHUNKS, file_size).await;
        });

       tasks.push(task); 
    }

    for task in tasks {
        task
        .await
        .map_err(|error| format!("{file_hash}: A chunk task failed: {error}"))?;
    }


    for chunk_num in 0..NUM_OF_CHUNKS {
        println!("{file_hash}: Merging chunk {chunk_num}");

        let chunk_path = chunk::get_chunk_path(file_hash, chunk_num).await?;

        let mut chunk_file = File
            ::open(&chunk_path)
            .await
            .map_err(|error| format!("{file_hash}: The chunk file is missing!: {error}"))?;

        io::copy(&mut chunk_file, &mut final_file)
            .await
            .map_err(|error| format!("{file_hash}: Copying the chunk file data to the final file failed: {error}"))?;

        fs::remove_file(&chunk_path)
         .await
         .map_err(|error| format!("{file_hash}: Failed to remove the chunk file: {error}"))?;
    }

    println!("Finished!");

    Ok(())
}

