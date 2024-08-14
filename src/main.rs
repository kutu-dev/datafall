use relm4::RelmApp;

use datafall::components::App;

fn main() {
    let app = RelmApp::new("dev.dobon.datafall");

    relm4_icons::initialize_icons();

    app.run::<App>(());
}
