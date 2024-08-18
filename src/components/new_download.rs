use relm4::{adw::prelude::*, gtk::prelude::*, prelude::*};

use reqwest::Url;

use relm4_icons::icon_names;

pub struct NewDownload {
    error_text: String,
}

#[derive(Debug)]
pub enum NewDownloadInput {
    Create,
    Close,
    InvalidUrl,
}

#[derive(Debug)]
pub enum NewDownloadOutput {
    Create(Url),
}

#[relm4::component(pub)]
impl Component for NewDownload {
    type Init = ();
    type Input = NewDownloadInput;
    type Output = NewDownloadOutput;
    type CommandOutput = ();

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            error_text: String::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        input: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match input {
            Self::Input::Create => {
                let entry = widgets.entry.text().to_string();
                let url = Url::parse(&entry);

                match url {
                    Ok(url) => {
                        sender
                            .output(Self::Output::Create(url))
                            .expect("Sending the URL to the main window failed!");
                        sender.input(Self::Input::Close);
                    }
                    Err(_) => {
                        sender.input(Self::Input::InvalidUrl);
                    }
                };
            }

            Self::Input::Close => {
                widgets.entry.set_text("");
                widgets.entry.remove_css_class("error");
                self.error_text = String::from("");
                root.close();
            }

            Self::Input::InvalidUrl => {
                self.error_text = String::from("The URL is invalid!");
                widgets.entry.add_css_class("error");
            }
        }

        self.update_view(widgets, sender);
    }

    view! {
        adw::Dialog {
            #[watch]
            set_title: "New Download",

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_show_start_title_buttons: false,
                    set_show_end_title_buttons: false,

                    pack_start = &gtk::Button::with_label("Cancel") {
                        connect_clicked => Self::Input::Close,
                    },

                    pack_end = &gtk::Button::with_label("Create") {
                        add_css_class: "suggested-action",
                        connect_clicked => Self::Input::Create,
                    },
                },

                #[name = "entry"]
                adw::EntryRow {
                    set_title: "Resource URL",

                    set_margin_all: 7,
                    set_input_purpose: gtk::InputPurpose::Url,
                },

                gtk::Label {
                    add_css_class: "error",
                    set_margin_bottom: 10,

                    #[watch]
                    set_label: &model.error_text,
                },
            },
        }
    }
}
