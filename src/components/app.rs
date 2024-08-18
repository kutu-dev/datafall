use relm4::{adw::prelude::*, factory::FactoryVecDeque, gtk::prelude::*, prelude::*};

use reqwest::{Client, Url};

use relm4_icons::icon_names;

use crate::components::{DownloadItem, NewDownload, NewDownloadOutput};

pub struct App {
    new_download: Controller<NewDownload>,
    download_factory: FactoryVecDeque<DownloadItem>,
    client: Client,
}

#[derive(Debug)]
pub enum AppInput {
    CreateNewDownload,
    StartNewDownload(Url),
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let new_download = NewDownload::builder().launch(()).forward(
            sender.input_sender(),
            |output| match output {
                NewDownloadOutput::Create(url) => Self::Input::StartNewDownload(url),
            },
        );

        let download_factory = FactoryVecDeque::builder()
            .launch_default()
            .forward(sender.input_sender(), |_| Self::Input::CreateNewDownload);

        let client = Client::new();

        let model = Self {
            new_download,
            download_factory,
            client,
        };

        let download_items = model.download_factory.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match input {
            Self::Input::CreateNewDownload => {
                self.new_download.widget().present(root);
            }

            Self::Input::StartNewDownload(url) => {
                self.download_factory.guard().push_back((self.client.clone(), url));
            }
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
                    },

                    pack_end = &gtk::Button {
                        set_icon_name: icon_names::MENU_LARGE
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
                        set_icon_name: Some(icon_names::FOLDER_OPEN_FILLED),
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
