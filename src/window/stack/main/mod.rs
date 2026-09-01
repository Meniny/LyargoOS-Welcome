// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/main/mod.rs
 * Copyright (C) 2026 LyargoOS
 */

use gio::prelude::*;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::{tweak::Tweak, window::Window};

mod imp;

glib::wrapper! {
    pub struct StackPageMain(ObjectSubclass<imp::StackPageMain>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl StackPageMain {
    fn handle_tweak(&self, tweak: Tweak) {
        let window = self.root().and_downcast::<Window>().unwrap();

        let stack_page_log = &window.imp().stack_page_log.imp();
        let buffer = &stack_page_log.text_view_log.buffer();

        window.emit_by_name::<()>("navigate", &[&"log"]);
        window.set_title(Some(tweak.title()));

        tweak.prompt(buffer, &*stack_page_log.obj());
    }
}
