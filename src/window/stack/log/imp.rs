// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/stack/log/imp.rs
 * Copyright (C) 2026 LyargoOS
 */

use std::cell::RefCell;

use crate::window::Window;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/lyargoos/Welcome/ui/stack/log.ui")]
pub struct StackPageLog {
    #[template_child]
    pub text_view_log: TemplateChild<gtk::TextView>,

    #[template_child]
    pub box_confirmation: TemplateChild<gtk::Box>,

    #[template_child]
    pub button_return: TemplateChild<gtk::Button>,

    pub current_tweak: RefCell<Option<Tweak>>,
}

#[gtk::template_callbacks]
impl StackPageLog {
    #[template_callback]
    fn on_button_return_clicked(&self) {
        let window = self.obj().root().and_downcast::<Window>().unwrap();

        window.emit_by_name::<()>("navigate", &[&"main"]);

        self.text_view_log.buffer().set_text("");
        self.button_return.set_visible(false)
    }

    #[template_callback]
    fn on_button_cancel_clicked(&self) {
        self.box_confirmation.set_visible(false);
        self.text_view_log
            .buffer()
            .set_text("Operation canceled.");
        self.button_return.set_visible(true)
    }

    #[template_callback]
    fn on_button_proceed_clicked(&self) {
        let buffer = self.text_view_log.buffer();
        let current_tweak = self.current_tweak.borrow().unwrap();

        current_tweak.run(
            buffer,
            &*self.obj(),
            glib::clone!(
                #[weak(rename_to = stack_page)]
                self,
                move || {
                    stack_page.button_return.set_visible(true);

                    let buffer = stack_page.text_view_log.buffer();
                    buffer.insert(&mut buffer.end_iter(), "Completed successfully\n");
                }
            ),
        );
    }
}

#[glib::object_subclass]
impl ObjectSubclass for StackPageLog {
    const NAME: &'static str = "StackPageLog";
    type Type = super::StackPageLog;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for StackPageLog {
    fn constructed(&self) {
        self.parent_constructed();
        self.text_view_log.set_monospace(true);
    }
}

impl WidgetImpl for StackPageLog {}
impl BoxImpl for StackPageLog {}
