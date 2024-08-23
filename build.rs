// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use glib_build_tools::compile_resources;

fn main() {
    compile_resources(
        &["data"],
        "./data/icons.gresource.xml",
        "icons.gresource",
    );
}
