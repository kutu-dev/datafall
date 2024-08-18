use relm4::{adw::prelude::*, gtk::prelude::*, prelude::*};
use chrono;
use reqwest::{Client, Response, Url};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use tokio::{
    fs::File,
    io,
};

use crate::{
    resource_metadata::ResourceMetadata,
    download_fragment,
    chunk,
};

//TODO: Should be moved to a new module (maybe with get_chunk_path)
fn get_filename(url: &Url) -> String {
    url.path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or(&chrono::offset::Utc::now().to_string().replace(" ", "-"))
        .to_owned()
}

async fn get_file(filename: &str) -> Result<File, String> {
    File::create(filename)
        .await
        .map_err(|error| format!("Couldn't create a new empty file: {error}"))
}

pub fn get_hash(file_size: u64, url: &Url) -> String {
    URL_SAFE.encode(format!("{url}-{file_size}"))
}

pub struct DownloadItem {
    filename: String,
    progress: f64,
    client: Client,
    url: Url,
    chunks_downloaded: u64,
}

#[derive(Debug)]
pub enum DownloadItemInput {
    SplitDownload { file_size: u64, num_of_chunks: u64 },
    PlainDownload,
    MergeChunks(String, u64),
}

#[derive(Debug)]
pub enum DownloadItemCommandOutput {
    ResourceMetadata(ResourceMetadata),
    ChunkDownloaded(String, u64),
    DownloadFinished,
    Error(String),
}

#[relm4::factory(pub)]
impl FactoryComponent for DownloadItem {
    type Init = (Client, Url);
    type Input = DownloadItemInput;
    type Output = ();
    type CommandOutput = DownloadItemCommandOutput;
    type ParentWidget = gtk::ListBox;

    fn init_model(init: Self::Init, _index: &DynamicIndex, sender: FactorySender<Self>) -> Self {
        let url = init.1;
        let filename = get_filename(&url);

        let model = Self {
            filename,
            progress: 0.0,
            client: init.0,
            url,
            chunks_downloaded: 0,
        };

        let client = model.client.clone();
        let url = model.url.clone();
        sender.oneshot_command(async move {
            ResourceMetadata::new(client, url)
                .await
                .map_or_else(
                    |error| Self::CommandOutput::Error(error),
                    |value| Self::CommandOutput::ResourceMetadata(value)
                )
        });

        model
    }

    fn update_cmd(&mut self, message: Self::CommandOutput, sender: FactorySender<Self>) {
        match message {
            Self::CommandOutput::ResourceMetadata(resource_metadata) => {
                self.progress += 0.1;

                // Check if the resource is eligible for a split download
                if let Some(size) = resource_metadata.size {
                    if resource_metadata.accept_ranges {
                        sender.input(Self::Input::SplitDownload {
                            file_size: size,
                            num_of_chunks: 16,
                        });
                        
                        return;
                    }
                }

                sender.input(Self::Input::PlainDownload);
            },

            Self::CommandOutput::ChunkDownloaded(file_hash, num_of_chunks) => {
                self.chunks_downloaded += 1;

                self.progress += 0.8 / num_of_chunks as f64;

                if self.chunks_downloaded == num_of_chunks {
                    sender.input(Self::Input::MergeChunks(file_hash, num_of_chunks));
                }
            },

            Self::CommandOutput::DownloadFinished => {
                self.progress = 1.0;
            },

            Self::CommandOutput::Error(error) => {
                //TODO
            },
        }
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            Self::Input::SplitDownload{file_size, num_of_chunks} => {
                let file_hash = get_hash(file_size, &self.url);
                let chunk_size = file_size / num_of_chunks;

                for chunk_num in 0..num_of_chunks {
                    let client = self.client.clone();
                    let url = self.url.clone();
                    let file_hash = file_hash.clone();

                    sender.oneshot_command(async move {
                        chunk::download_chunk(client, url, chunk_num, num_of_chunks, chunk_size, &file_hash, file_size)
                            .await
                            .map_or_else(
                                |error| Self::CommandOutput::Error(error),
                                |_| Self::CommandOutput::ChunkDownloaded(file_hash, num_of_chunks)
                            )
                    })
                }
            }

            Self::Input::PlainDownload => {
                let client = self.client.clone();
                let url = self.url.clone();
                let filename = self.filename.clone();

                sender.oneshot_command(async move {
                    let file = get_file(&filename).await;

                    if let Err(error) = file {
                        return Self::CommandOutput::Error(error);
                    }
                    let file = file.unwrap();
                    
                    let result = download_fragment::download_fragment(&client, url, file, None).await;

                    if let Err(error) = result {
                        return Self::CommandOutput::Error(error);
                    }

                    Self::CommandOutput::DownloadFinished
                });
            },

            Self::Input::MergeChunks(file_hash, num_of_chunks) => {
                let filename = self.filename.clone();

                sender.oneshot_command(async move {
                    let final_file = get_file(&filename).await;

                    if let Err(error) = final_file {
                        return Self::CommandOutput::Error(error);
                    };
                    let mut final_file = final_file.unwrap();

                    for chunk_num in 0..num_of_chunks {
                        let chunk_path = chunk::get_chunk_path(&file_hash, chunk_num).await;
                        
                        if let Err(error) = chunk_path {
                            return Self::CommandOutput::Error(error);
                        };
                        let chunk_path = chunk_path.unwrap();

                        let chunk_file = File
                            ::open(&chunk_path)
                            .await
                            .map_err(|error| format!("The chunk file is missing!: {error}"));

                        if let Err(error) = chunk_file {
                            return Self::CommandOutput::Error(error);
                        };
                        let mut chunk_file = chunk_file.unwrap();

                        let result = io::copy(&mut chunk_file, &mut final_file)
                            .await
                            .map_err(|error| format!("Copying the chunk file data to the final file failed: {error}"));

                        if let Err(error) = result {
                            return Self::CommandOutput::Error(error);
                        };
                    }

                    Self::CommandOutput::DownloadFinished
                });
            },
        }
    }

    view! {
        gtk::ListBoxRow{
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Label {
                    set_label: &self.filename,
                },

                gtk::ProgressBar {
                    #[watch]
                    set_fraction: self.progress,
                },
            }
        },
    }
}
