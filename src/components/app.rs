use relm4::{
    prelude::*,
    gtk::prelude::*,
    adw::prelude::*,
};

use relm4_icons::icon_names;

pub struct App {}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = ();
    type Output = ();

    fn init(_init: Self::Init, _root: Self::Root, _sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        adw::ApplicationWindow {
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::ViewSwitcher {
                        set_policy: adw::ViewSwitcherPolicy::Wide,
                        set_stack: Some(&stack),
                    },
                },

                #[name = "stack"]
                adw::ViewStack {
                    set_vexpand: true,

                    add_titled[Some("Queue"), "Queue"] = &gtk::Box{
                        gtk::Button {
                            set_icon_name: icon_names::PLUS
                        }
                    } -> {
                        set_icon_name: Some(icon_names::FOLDER_OPEN_FILLED),
                    },

                    add_titled[Some("History"), "History"] = &gtk::Label {
                        set_label: "TWO",
                    } -> {
                        set_icon_name: Some(icon_names::CLOCK),
                    },
                },
            }
        },
    }
}
