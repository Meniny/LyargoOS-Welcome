/* main.rs
 *
 * Copyright 2026 LyargoOS
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod application;
mod config;
mod tweak;
mod util;
mod window;

use application::Application;
use gtk::glib;
use gtk::prelude::*;

use config::PKGDATADIR;

const APP_ID: &str = "org.lyargoos.Welcome";

fn main() -> glib::ExitCode {
    let resources = gio::Resource::load(format!("{}/resources.gresource", PKGDATADIR))
        .expect("Could not load resources");
    gio::resources_register(&resources);

    // Create a new application
    let app = Application::new(APP_ID, &gio::ApplicationFlags::empty());

    // Run the application
    app.run()
}
