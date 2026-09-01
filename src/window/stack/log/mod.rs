// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/log/mod.rs
 * Copyright (C) 2026 LyargoOS
 */

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::tweak::Tweak;
use crate::tweak::TweakLogger;

mod imp;

glib::wrapper! {
    pub struct StackPageLog(ObjectSubclass<imp::StackPageLog>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::ConstraintTarget, gtk::Buildable, gtk::Accessible;
}

impl TweakLogger for StackPageLog {
    fn set_tweak(&self, tweak: Tweak) {
        *self.imp().current_tweak.borrow_mut() = Some(tweak)
    }

    fn show_confirmation(&self) {
        self.imp().box_confirmation.set_visible(true);
    }

    fn hide_confirmation(&self) {
        self.imp().box_confirmation.set_visible(false);
    }

    fn show_return(&self) {
        self.imp().button_return.set_visible(true);
    }
}
