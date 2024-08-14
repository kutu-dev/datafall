use relm4::RelmApp;

use datafall::components::App;

use relm4::prelude::*;
use relm4::gtk::prelude::*;
use relm4_icons::icon_names;

fn main() {
    let app = RelmApp::new("dev.dobon.datafall");

    relm4_icons::initialize_icons();

    app.run::<App>(());
}
