// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/mod.rs
 * Copyright (C) 2026 LyargoOS
 */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

pub use stack::{log::StackPageLog, main::StackPageMain};

mod imp;
mod stack;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible, gtk::ShortcutManager, gtk::Root, gtk::Native;
}

impl Window {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn connect_navigate<F: Fn(&Self, &str) + 'static>(&self, f: F) -> glib::SignalHandlerId {
        self.connect_closure(
            "navigate",
            true,
            glib::closure_local!(move |obj: Self, stackpage: &str| {
                f(&obj, stackpage);
            }),
        )
    }
}
