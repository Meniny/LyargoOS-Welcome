// SPDX-License-Identifier: GPL-3.0-or-later
/* src/tweak.rs
 * Copyright (C) 2026 LyargoOS
 */

use std::ffi::OsStr;

use gio::glib;
use gtk::prelude::*;

use crate::util::{read_line_utf8_async_to_buffer, scripts_dir};

const HEADER: &str = r#"
 _                                 ___  ____  
| |   _   _  __ _ _ __ __ _  ___  / _ \/ ___| 
| |  | | | |/ _` | '__/ _` |/ _ \| | | \___ \ 
| |__| |_| | (_| | | | (_| | (_) | |_| |___) |
|_____\__, |\__,_|_|  \__, |\___/ \___/|____/ 
      |___/           |___/                   
"#;

#[derive(Debug, Copy, Clone)]
pub enum Tweak {
    SystemUpdate,
    VirtManager,
}

pub trait TweakLogger: glib::clone::Downgrade {
    fn set_tweak(&self, tweak: Tweak);
    fn show_confirmation(&self);
    fn hide_confirmation(&self);
    fn show_return(&self);
}

impl Tweak {
    pub fn title(&self) -> &'static str {
        match self {
            Self::SystemUpdate => "System update",
            Self::VirtManager => "Install virt-manager",
        }
    }

    fn summary(&self) -> &'static str {
        match self {
            Self::SystemUpdate => {
                r#"
This will update your system using the XBPS package manager.

"#
            }
            Self::VirtManager => {
                r#"
This will install virt-manager. In the process it will:
- Install these packages: qemu, virt-manager, virt-viewer, dnsmasq, vde2,
  bridge-utils, openbsd-netcat, libguestfs
- Enable these Runit services: libvirtd, virtlogd
- Modify these files: /etc/libvirt/libvirtd.conf, /etc/libvirt/qemu.conf
- Add your user to the libvirt and kvm groups.

You will need to restart for changes to take effect.

"#
            }
        }
    }

    fn script(&self) -> std::path::PathBuf {
        let filename = match self {
            Self::SystemUpdate => "system_update.sh",
            Self::VirtManager => "virt_manager.sh",
        };

        scripts_dir().join(filename)
    }

    pub fn prompt(&self, buffer: &gtk::TextBuffer, logger: &impl TweakLogger) {
        logger.set_tweak(*self);
        logger.show_confirmation();

        buffer.set_text(HEADER);
        buffer.insert(&mut buffer.end_iter(), self.summary());
    }

    pub fn run<F>(&self, buffer: gtk::TextBuffer, logger: &impl TweakLogger, on_done: F)
    where
        F: Fn() + 'static,
    {
        logger.hide_confirmation();

        let script = &self.script();
        let script = script.to_string_lossy();

        let argv = &[
            OsStr::new("pkexec"),
            OsStr::new("bash"),
            OsStr::new("-c"),
            OsStr::new(&*script),
        ];
        let subprocess = gio::Subprocess::newv(
            argv,
            gio::SubprocessFlags::STDOUT_PIPE.union(gio::SubprocessFlags::STDERR_MERGE),
        )
        .unwrap();
        let stream = gio::DataInputStream::new(&subprocess.stdout_pipe().unwrap());

        read_line_utf8_async_to_buffer(stream, buffer, on_done);
    }
}
