// SPDX-License-Identifier: GPL-3.0-or-later
/* src/application/imp.rs
 * Copyright (C) 2026 LyargoOS
 */

use crate::window::Window;

use super::*;

#[derive(Debug, Default)]
pub struct Application {}

impl Application {
    fn present_main_window(&self) -> Window {
        let application = self.obj();

        let window: Window = if let Some(window) = application.active_window().and_downcast() {
            window
        } else {
            Window::new(&*application).upcast()
        };

        window.present();
        window
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Application {
    const NAME: &'static str = "Application";
    type Type = super::Application;
    type ParentType = gtk::Application;
}

impl ObjectImpl for Application {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_accels_for_action("app.quit", &["<primary>q"]);
    }
}

impl ApplicationImpl for Application {
    fn startup(&self) {
        self.parent_startup();
    }

    fn activate(&self) {
        self.parent_activate();
        self.present_main_window();
    }
}

impl GtkApplicationImpl for Application {}
