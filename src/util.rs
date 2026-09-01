// SPDX-License-Identifier: GPL-3.0-or-later
/* src/util.rs
 * Copyright (C) 2026 LyargoOS
 */

use gio::prelude::*;
use gtk::prelude::*;

use crate::config::SCRIPTSDIR;

pub fn autostart_file() -> std::path::PathBuf {
    gio::glib::home_dir()
        .join(".config")
        .join("autostart")
        .join("org.lyargoos.Welcome.desktop")
}

pub fn scripts_dir() -> std::path::PathBuf {
    let path = std::path::PathBuf::from(SCRIPTSDIR);
    #[cfg(debug_assertions)]
    {
        path.canonicalize().unwrap_or(path)
    }
    #[cfg(not(debug_assertions))]
    {
        path
    }
}

pub fn read_line_utf8_async_to_buffer<F>(
    stream: gio::DataInputStream,
    buffer: gtk::TextBuffer,
    on_done: F,
) where
    F: Fn() + 'static,
{
    stream.read_line_utf8_async(
        gio::glib::Priority::DEFAULT,
        gio::Cancellable::NONE,
        gio::glib::clone!(
            #[strong]
            stream,
            move |result| {
                if let Ok(Some(line)) = result {
                    buffer.insert(&mut buffer.end_iter(), &format!("{line}\n"));
                    read_line_utf8_async_to_buffer(stream, buffer, on_done);
                } else {
                    on_done();
                }
            }
        ),
    );
}
