use datafall;

use relm4::prelude::*;
use relm4::gtk::prelude::*;

struct App {
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = ();
    type Output = ();
    type Init = ();
    
    view! {
        gtk::Window {
            set_title: Some("DataFall"),
            set_default_width: 300,
            set_default_height: 100,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_top: 20,
                set_margin_start: 40,
                set_margin_end: 40,

                gtk::ListBox{
                    set_selection_mode: gtk::SelectionMode::None,

                    gtk::ListBoxRow {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            gtk::Label {
                                set_label: "Monty Python",
                            },

                            gtk::ProgressBar {
                                set_fraction: 0.5,
                            },
                        }
                    },
                    gtk::ListBoxRow {
                        gtk::Label {
                            set_label: "TEST",
                        },

                        gtk::ProgressBar {
                            set_fraction: 0.5,
                        },
                    },
                },
            }
        }
    }

    fn init(
        _: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = App{};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
    }
}

#[tokio::main]
async fn main() {
    let app = RelmApp::new("dev.dobon.datafall");
    app.run::<App>(());
}
