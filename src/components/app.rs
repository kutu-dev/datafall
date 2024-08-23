// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use relm4::{adw::prelude::*, factory::FactoryVecDeque, prelude::*};
use relm4::actions::{RelmAction, RelmActionGroup};

use reqwest::{Client, Url};

use relm4_icons::icon_names;

use crate::components::{DownloadItem, DownloadItemOutput, NewDownload, NewDownloadOutput, AboutDialog, CleanCache};

relm4::new_action_group!(MainActionGroup, "main");
relm4::new_stateless_action!(OpenAboutAction, MainActionGroup, "open-about");
relm4::new_stateless_action!(CleanCacheAction, MainActionGroup, "clean-cache");

pub struct App {
    new_download: Controller<NewDownload>,
    about_dialog: Controller<AboutDialog>,
    clean_cache: Controller<CleanCache>,
    download_factory: FactoryVecDeque<DownloadItem>,
    client: Client,
}

#[derive(Debug)]
pub enum AppInput {
    CreateNewDownload,
    StartNewDownload(Url),
    CancelDownload(DynamicIndex),
    OpenAbout,
    CleanCache,
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let new_download = NewDownload::builder().launch(()).forward(
            sender.input_sender(),
            |output| match output {
                NewDownloadOutput::Create(url) => Self::Input::StartNewDownload(url),
            },
        );

        let about_dialog = AboutDialog::builder().launch(()).detach();
        let clean_cache = CleanCache::builder().launch(()).detach();

        let download_factory =
            FactoryVecDeque::builder()
                .launch_default()
                .forward(sender.input_sender(), |output| match output {
                    DownloadItemOutput::Cancel(index) => Self::Input::CancelDownload(index),
                });

        let client = Client::new();

        let model = Self {
            new_download,
            about_dialog,
            clean_cache,
            download_factory,
            client,
        };

        let download_items = model.download_factory.widget();

        let widgets = view_output!();

        let cloned_sender = sender.clone();
        let open_about_action: RelmAction<OpenAboutAction> = {
            RelmAction::new_stateless(move |_| {
                cloned_sender.input(Self::Input::OpenAbout);
            })
        };

        let clean_cache_action: RelmAction<CleanCacheAction> = {
            RelmAction::new_stateless(move |_| {
                sender.input(Self::Input::CleanCache);
            })
        };

        let mut group = RelmActionGroup::<MainActionGroup>::new();

        group.add_action(open_about_action);
        group.add_action(clean_cache_action);

        group.register_for_widget(&widgets.main_window);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match input {
            Self::Input::CreateNewDownload => {
                self.new_download.widget().present(root);
            }

            Self::Input::StartNewDownload(url) => {
                self.download_factory
                    .guard()
                    .push_back((self.client.clone(), url));
            }

            Self::Input::CancelDownload(index) => {
                self.download_factory.guard().remove(index.current_index());
            },

            Self::Input::OpenAbout => {
                self.about_dialog.widget().present(root);
            },

            Self::Input::CleanCache => {
                self.clean_cache.widget().present(root);
            },
        }
    }

    menu! {
        main_menu: {
            custom: "widget",
            "Clean cache" => CleanCacheAction,
            "About DataFall" => OpenAboutAction,
        }
    }

    view! {
        main_window = adw::ApplicationWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcher {
                        set_policy: adw::ViewSwitcherPolicy::Wide,
                        set_stack: Some(&stack),
                    },

                    pack_start = &gtk::Button {
                        set_icon_name: icon_names::PLUS_LARGE,
                        connect_clicked => Self::Input::CreateNewDownload,
                        set_tooltip_text: Some("Start a new download"),
                    },

                    pack_end = &gtk::MenuButton {
                        set_icon_name: icon_names::MENU_LARGE,
                        set_tooltip_text: Some("Open application menu"),
                        
                        #[wrap(Some)]
                        set_popover = &gtk::PopoverMenu::from_model(Some(&main_menu)) {}
                    },
                },

                #[name = "stack"]
                adw::ViewStack {
                    set_vexpand: true,

                    add_titled[Some("Queue"), " Queue"] = &gtk::ScrolledWindow {
                        #[local_ref]
                         download_items -> gtk::ListBox {
                            add_css_class: "boxed-list-separate",
                            set_selection_mode: gtk::SelectionMode::None,
                        }
                    } -> {
                        set_icon_name: Some(icon_names::ARROW4_DOWN),
                    },

                    add_titled[Some("History"), " History"] = &gtk::Label {
                        set_label: "WORK IN PROGRESS",
                    } -> {
                        set_icon_name: Some(icon_names::CLOCK),
                    },
                },
            }
        },
    }
}
