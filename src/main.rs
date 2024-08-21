// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use relm4::RelmApp;

use datafall::components::App;

use relm4::gtk::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_names;

fn main() {
    let app = RelmApp::new("dev.dobon.datafall");

    relm4_icons::initialize_icons();

    app.run::<App>(());
}
