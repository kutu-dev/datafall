// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use relm4::prelude::*;



const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct AboutDialog {}

#[relm4::component(pub)]
impl Component for AboutDialog{
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update (
        &mut self,
        _input: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {}

    view! {
        adw::AboutDialog {
            set_application_name: "DataFall",
            set_application_icon: "icon",
            set_developer_name: "Jorge \"Kutu\" Dobón Blanco",
            set_version: VERSION,
            set_website: "https://github.com/kutu-dev/datafall",
            set_issue_url: "https://github.com/kutu-dev/datafall/issues",
            set_license_type: gtk::License::Mpl20,
        }
    }
}
