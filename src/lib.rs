mod file_metadata;
mod download_fragment;
mod split_download;
mod chunk;

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

use crate::file_metadata::FileMetadata;

pub fn download_file(url: String) -> Result<(), String> {
    let client = Client::new();

    let url = Url
        ::parse(&url)
        .map_err(|error| format!("The URL is invalid: {error}"))?;

    let file_metadata = FileMetadata::new(
        &client,
        url.clone(),
    )?;
    
    let file = File
        ::create(&file_metadata.filename)
        .map_err(|error| format!("Couldn't create a new empty file: {error}"))?;

    if let Some(size) = file_metadata.size {
        if file_metadata.accept_ranges {
            println!("Resource eligible for split download");
            split_download::split_download(&client, url.clone(), file, size)?;

            return Ok(());
        }
    }

    println!("Resource not eligible for split download");
    download_fragment::download_fragment(&client, url.clone(), file, None)?;

    Ok(())
}
