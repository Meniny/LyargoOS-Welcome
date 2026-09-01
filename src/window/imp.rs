// SPDX-License-Identifier: GPL-3.0-or-later
/* src/window/imp.rs
 * Copyright (C) 2026 LyargoOS
 */

use std::sync::LazyLock;

use gio::glib::subclass::Signal;

use super::*;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(resource = "/org/lyargoos/Welcome/ui/window.ui")]
pub struct Window {
    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,

    #[template_child]
    pub stack_page_main: TemplateChild<StackPageMain>,

    #[template_child]
    pub stack_page_log: TemplateChild<StackPageLog>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "Window";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        StackPageMain::ensure_type();
        StackPageLog::ensure_type();

        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().connect_navigate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |_widget, stackpage| {
                match stackpage {
                    "main" => window.obj().set_title(Some("Welcome to LyargoOS")),
                    _ => {}
                }

                window.stack.set_visible_child_name(stackpage);
            }
        ));
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: LazyLock<Vec<Signal>> = LazyLock::new(|| {
            vec![
                Signal::builder("navigate")
                    .param_types([String::static_type()])
                    .build(),
            ]
        });

        SIGNALS.as_ref()
    }
}

impl WidgetImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
