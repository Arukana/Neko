// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed

//! # neko
//!
//! This library contains the module `graphic` and `dynamic`.

#![feature(range_contains)]
#![feature(slice_patterns)]
#![feature(untagged_unions)]
#![feature(advanced_slice_patterns)]
#![feature(result_unwrap_or_default)]

#![crate_type= "lib"]
#![cfg_attr(feature = "nightly", feature(plugin))]

#![feature(plugin)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file="clippy.toml")))]

#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]
#![deny(
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications
)]

#![doc(html_logo_url = "https://arukana.github.io/Neko/images/neko.png")]

extern crate pty_proc;
extern crate graphic;
extern crate git2;
extern crate toml;
extern crate libc;

#[macro_use]
/// The macros of crate.
mod macros;
/// The module `display` is a extension of pty-proc's display module.
pub mod display;
/// The module `prelude` is for public.
pub mod prelude;
/// The module `dynamic` is the compositer of extern libraries.
pub mod dynamic;

mod err;

use std::io::{self, Write};
use std::fmt;

use dynamic::Compositer;
use dynamic::library::state::LibraryState;
use display::Display;

pub use graphic::prelude as editeur;
pub use pty_proc::prelude as pty;

pub use self::err::{NekoError, Result};

/// The first directory.
pub const SPEC_ROOT: &'static str = editeur::SPEC_ROOT;


/// The module `neko` is the first interface level.
pub struct Neko {
    dynamic: Compositer,
    /// Overload of Display interface from Pty.
    display: Display,
    /// Interface of Pseudo terminal.
    shell: pty::Shell,
    /// Interface on a Sprite partition.
    graphic: editeur::Graphic,
    /// Command
    command: io::Cursor<Vec<char>>,
    pid: libc::pid_t,
}

impl Neko {
    pub fn new(repeat: Option<i64>, interval: Option<i64>) -> Result<Self> {
        let dynamic: Compositer = try!(Compositer::new());
        let shell: pty::Shell = try!(pty::Shell::new(repeat, interval, None));
        let graphic: editeur::Graphic = try!(editeur::Graphic::new());
        let pid = shell.get_pid();

        let mut neko = Neko {
            display: Display::default(),
            dynamic: dynamic,
            shell: shell,
            graphic: graphic,
            command: io::Cursor::new(Vec::new()),
            pid: pid,
        };
        neko.call();
        Ok(neko)
    }

    #[allow(unused_must_use)]
    fn command(&mut self, key: pty::Key) {
        let position: u64 = self.command.position();
        match key {
            pty::Key::Utf8(glyph) => {
                self.command.get_mut().insert(position as usize,  glyph);
                self.command.set_position(position.checked_add(1).unwrap_or_default());
            },
            pty::Key::Left => {
                self.command.set_position(position.checked_sub(1).unwrap_or_default());
            },
            pty::Key::Right => {
                self.command.set_position(position.checked_add(1).unwrap_or_default());
            },
            pty::Key::Enter => {
                match &self.command.get_ref().iter().cloned().collect::<String>()
                                   .to_string().split_whitespace().collect::<Vec<&str>>()
                                   .as_slice()[..] {
                    &["neko", "install", ref repository] => {
                        self.dynamic.install(repository)
                    },
                    &["neko", "mount", ref libraryname, ref priority] => {
                        self.dynamic.mount(libraryname, priority.parse::<i64>().ok())
                    },
                    &["neko", "mount", ref libraryname] => {
                        self.dynamic.mount(libraryname, None)
                    },
                    &["neko", "unmount", ref libraryname] => {
                        self.dynamic.unmount(libraryname)
                    },
                    &["neko", "uninstall", ref libraryname] => {
                        self.dynamic.uninstall(libraryname)
                    },
                    &["neko", "update", ref libraryname] => {
                        self.dynamic.update(libraryname)
                    },
                    _ => Ok(()),
                };
                self.command.get_mut().clear();
                self.command.set_position(0);
            },
            _ => {
                self.command.get_mut().clear();
                self.command.set_position(0);
            },
        };
    }

    fn call(&mut self) {
        let lib: &LibraryState = self.dynamic.get_state();
 
        let screen: &pty::Display = self.shell.get_screen();
        if let Some(sprite) = self.graphic.explicite_emotion(
            lib.get_sheet(),
            lib.get_explicite(),
        ) {
            self.display.with_draw(
                screen,
                sprite.into_iter().next().unwrap(),
                lib.get_message(),
                lib.get_position().get_coordinate(screen.get_window_size()),
            );
        }
    }

    /// The accessor method `get_screen` returns a reference on the Display interface.
    pub fn get_screen(&self) -> &Display {
        &self.display
    }
}

impl Iterator for Neko {
    type Item = pty::ShellState;

    fn next(&mut self) -> Option<pty::ShellState> {
        self.shell.next().and_then(|event| {
            if let Some(&(pid, _)) = event.is_task() {
                self.pid = pid;
            }
            if let Some(key) = event.is_input_keydown() {
                if self.pid.eq(&self.shell.get_pid()) {
                    self.command(key);
                }
            }
            self.dynamic.call(&event);
            self.call();
            Some(event)
        })
    }
}

impl Write for Neko {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.shell.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.shell.flush()
    }
}

impl fmt::Display for Neko {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_screen()
                            .into_iter()
                            .map(|character: (&pty::Character)|
                                    character.get_glyph())
                            .collect::<String>())
    }
}

impl fmt::Debug for Neko {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Neko {{ dynamic: {:?}, graphic: {:?}, display: {:?} }}",
               self.dynamic,
               self.graphic,
               self.display)
    }
}
