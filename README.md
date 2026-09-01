# LyargoOS Welcome

Welcome screen for LyargoOS Linux, providing quick access to system updates and virtualization setup.

## Technical Details

Built with **Rust** and **GTK4** (via [gtk4-rs](https://gtk-rs.org/)) — the same toolkit used by GNOME. UI layouts are written in [Blueprint](https://jwestman.pages.gitlab.gnome.org/blueprint-compiler/) format (`.blp`) and compiled to GTK's XML format at build time.

## Features

- System update via XBPS package manager
- One-click virt-manager installation and configuration

## Build

Requires: `cargo`, `blueprint-compiler`, `gtk4`, `glib2`

**Easy way** (auto-installs dependencies):
```bash
./build.sh
```

**Manual way**:
```bash
./configure --prefix=/usr
make
sudo make install
```

## License

[![](https://www.gnu.org/graphics/gplv3-127x51.png)](https://www.gnu.org/licenses/gpl-3.0.txt)

This project is released under the GPL-3.0 License.
