// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use chrono;
use relm4::{adw::prelude::*, prelude::*};
use relm4_icons::icon_names;
use reqwest::{Client, Url};
use tokio::{fs::File, io};
use dirs;

use crate::{chunk, download_fragment, resource_metadata::ResourceMetadata};

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
    status: String,
    progress: f64,
    client: Client,
    url: Url,
    chunks_downloaded: u64,
    download_finished: bool,
}

#[derive(Debug)]
pub enum DownloadItemInput {
    SplitDownload { file_size: u64, num_of_chunks: u64 },
    PlainDownload,
    MergeChunks(String, u64),
    OpenFile,
}

#[derive(Debug)]
pub enum DownloadItemOutput {
    Cancel(DynamicIndex),
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
    type Output = DownloadItemOutput;
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
            status: String::from("Getting resource metadata..."),
            download_finished: false,
        };

        let client = model.client.clone();
        let url = model.url.clone();
        sender.oneshot_command(async move {
            ResourceMetadata::new(client, url).await.map_or_else(
                |error| Self::CommandOutput::Error(error),
                |value| Self::CommandOutput::ResourceMetadata(value),
            )
        });

        model
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: FactorySender<Self>,
    ) {
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
            }

            Self::CommandOutput::ChunkDownloaded(file_hash, num_of_chunks) => {
                self.chunks_downloaded += 1;

                self.progress += 0.8 / num_of_chunks as f64;

                if self.chunks_downloaded == num_of_chunks {
                    sender.input(Self::Input::MergeChunks(file_hash, num_of_chunks));
                }
            }

            Self::CommandOutput::DownloadFinished => {
                self.status = String::from("Downloading finished!");
                widgets.subtitle.add_css_class("success");
                self.progress = 1.0;
                self.download_finished = true;
            }

            Self::CommandOutput::Error(error) => {
                self.status = error;
                widgets.subtitle.add_css_class("error");
            }
        }

        self.update_view(widgets, sender);
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            Self::Input::SplitDownload {
                file_size,
                num_of_chunks,
            } => {
                self.status = String::from("Downloading the resource fragments...");

                let file_hash = get_hash(file_size, &self.url);
                let chunk_size = file_size / num_of_chunks;

                for chunk_num in 0..num_of_chunks {
                    let client = self.client.clone();
                    let url = self.url.clone();
                    let file_hash = file_hash.clone();

                    sender.oneshot_command(async move {
                        chunk::download_chunk(
                            client,
                            url,
                            chunk_num,
                            num_of_chunks,
                            chunk_size,
                            &file_hash,
                            file_size,
                        )
                        .await
                        .map_or_else(
                            |error| Self::CommandOutput::Error(error),
                            |_| Self::CommandOutput::ChunkDownloaded(file_hash, num_of_chunks),
                        )
                    })
                }
            }

            Self::Input::PlainDownload => {
                self.status = String::from("Downloading the resource...");

                let client = self.client.clone();
                let url = self.url.clone();
                let filename = self.filename.clone();

                sender.oneshot_command(async move {
                    let file = get_file(&filename).await;

                    if let Err(error) = file {
                        return Self::CommandOutput::Error(error);
                    }
                    let file = file.unwrap();

                    let result =
                        download_fragment::download_fragment(&client, url, file, None).await;

                    if let Err(error) = result {
                        return Self::CommandOutput::Error(error);
                    }

                    Self::CommandOutput::DownloadFinished
                });
            }

            Self::Input::MergeChunks(file_hash, num_of_chunks) => {
                self.status = String::from("Merging the file fragments...");
                let filename = self.filename.clone();

                sender.oneshot_command(async move {
                    let final_file = get_file(&filename).await;

                    if let Err(error) = final_file {
                        return Self::CommandOutput::Error(error);
                    };
                    let mut final_file = final_file.unwrap();

                    for chunk_num in 0..num_of_chunks {
                        let chunk_path = chunk::get_chunk_path(&file_hash, chunk_num, false).await;

                        if let Err(error) = chunk_path {
                            return Self::CommandOutput::Error(error);
                        };
                        let chunk_path = chunk_path.unwrap();

                        let chunk_file = File::open(&chunk_path)
                            .await
                            .map_err(|error| format!("The chunk file is missing!: {error}"));

                        if let Err(error) = chunk_file {
                            return Self::CommandOutput::Error(error);
                        };
                        let mut chunk_file = chunk_file.unwrap();

                        let result =
                            io::copy(&mut chunk_file, &mut final_file)
                                .await
                                .map_err(|error| {
                                    format!(
                                    "Copying the chunk file data to the final file failed: {error}"
                                )
                                });

                        if let Err(error) = result {
                            return Self::CommandOutput::Error(error);
                        };
                    }

                    Self::CommandOutput::DownloadFinished
                });
            },

            Self::Input::OpenFile => {
                let download_dir = dirs::download_dir();
                if let None = download_dir {
                    println!("Could not find the download directory path");
                    return;
                };
                let download_dir = download_dir.unwrap();

                let download_dir_uri = format!("file://{}", download_dir.display());
                let app_launch_context = gtk::gio::AppLaunchContext::new();

                let result = gtk::gio::AppInfo::launch_default_for_uri(&download_dir_uri, Some(&app_launch_context));

                if let Err(error) = result {
                    println!("Could not open the download directory: {error:?}");
                }
            }
        }
    }

    view! {
            gtk::ListBoxRow{
                gtk::CenterBox {
                    #[wrap(Some)]
                    set_center_widget = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_margin_all: 20,
                        set_spacing: 10,

                        gtk::CenterBox {
                            #[wrap(Some)]
                            set_start_widget = &gtk::Box {
                                // I could be better to find a way to set this
                                // values as percentage
                                set_size_request: (300, 0),

                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 5,

                                gtk::Label {
                                    set_label: &self.filename,
                                    set_halign: gtk::Align::Start,
                                    add_css_class: "title",
                                },

                                #[name = "subtitle"]
                                gtk::Label {
                                    #[watch]
                                    set_label: &self.status,
                                    set_halign: gtk::Align::Start,
                                    add_css_class: "subtitle",
                                },
                            },

                            #[wrap(Some)]
                            set_end_widget = if !&self.download_finished {
                                &gtk::Button {
                                    set_icon_name: icon_names::CROSS_LARGE,
                                    set_tooltip_text: Some("Cancel the download"),
                                    set_size_request: (40, 40),
                                    connect_clicked[sender, index] => move |_| {
                                        sender
                                            .output(Self::Output::Cancel(index.clone()))
                                            .expect("Unable to send cancel message to the parent component");
                                    },
                                }
                            } else {
                                &gtk::Button {
                                    set_icon_name: icon_names::FOLDER_OPEN_FILLED,
                                    set_tooltip_text: Some("Open file location"),
                                    connect_clicked => Self::Input::OpenFile,
                                }
                                
                            },
                        },

                        gtk::ProgressBar {
                            #[watch]
                            set_fraction: self.progress,
                        },
                },
            },
        }
    }
}
