// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use relm4::{adw::prelude::*, gtk::prelude::*, prelude::*};
use reqwest::Url;
use relm4_icons::icon_names;
use fs_extra::dir;
use std::path::PathBuf;
use std::fs;

use crate::chunk;

#[derive(Debug)]
pub struct CleanCache {
    body: String,
    cache_path: PathBuf,
}

#[derive(Debug)]
pub enum CleanCacheInput {
    Accept,
    Refresh
}

impl CleanCache {
    fn regenerate_body(&mut self){
        let mut cache_size_format = String::from("N/A ");

        let cache_size = dir::get_size(&self.cache_path);
        match cache_size {
            Ok(cache_size) => {
                // Bytes to megabytes (1MiB = 2^20 bytes)
                let cache_size = cache_size as f64 / 2_u64.pow(20) as f64;

                cache_size_format = format!("{cache_size:.2}");
            },

            Err(error) => {
                println!("Unable to get the size of the cache dir: {error}");
            }
        };

        self.body = format!("{cache_size_format}MiB of cached downloaded files will be removed.")
    }
}

#[relm4::component(pub)]
impl Component for CleanCache {
    type Init = ();
    type Input = CleanCacheInput;
    type Output = ();
    type CommandOutput = ();

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let cache_path = chunk::get_cache_path().expect("Unable to ge the cache path");
        let body = String::new();

        let mut model = Self {
            body,
            cache_path,
        };

        model.regenerate_body();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update (
        &mut self,
        input: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match input {
            Self::Input::Accept => {
                if let Err(error) = fs::remove_dir_all(&self.cache_path) {
                    println!("Unable to remove the cache dir");
                };

                if let Err(error) = fs::create_dir(&self.cache_path) {
                    println!("Unable to create a new cache dir");
                };

                self.regenerate_body();
            },

            Self::Input::Refresh => {
                self.regenerate_body();
            }
        };
    }

    view! {
        adw::AlertDialog {
            set_heading: Some("Do you want to clean the cache?"),

            #[watch]
            set_body: &model.body,

            add_response: ("cancel", "Cancel"),
            add_response: ("accept", "Clean"),

            connect_focus_widget_notify => Self::Input::Refresh,

            connect_response: (None, move |_, response| {
                if response == "accept" {
                    sender.input(Self::Input::Accept);
                };
            }),
        }
    }
}
