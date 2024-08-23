// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use relm4::RelmApp;

use datafall::components::App;

use relm4::gtk::{
    prelude::*,
    gdk,
    gio,
};
use relm4::prelude::*;

fn initialize_custom_icons() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/dev/dobon/datafall/icons");
}

fn main() {
    let app = RelmApp::new("dev.dobon.datafall");

    relm4_icons::initialize_icons();
    initialize_custom_icons();

    app.run::<App>(());
}
