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
use std::ops::{Not, BitAnd};
use std::fmt;
use std::char;

use dynamic::Compositer;
use dynamic::library::state::LibraryState;
use display::Display;

pub use graphic::prelude as editeur;
pub use pty_proc::prelude as pty;

pub use self::err::{NekoError, Result};

/// The first directory.
pub const SPEC_ROOT: &'static str = editeur::SPEC_ROOT;
/// The default first directory.
pub const SPEC_ROOT_DEFAULT: &'static str = editeur::SPEC_ROOT_DEFAULT;

/// The module `neko` is the first interface level.
pub struct Neko {
    dynamic: Compositer,
    /// Overload of Display interface from Pty.
    display: Display,
    /// Interface of Pseudo terminal.
    shell: pty::Shell,
    /// Interface on a Sprite partition.
    graphic: editeur::Graphic,
    /// The current pid.
    pid: libc::pid_t,
    /// The current Command.
    line: io::Cursor<Vec<char>>,
}

impl Neko {
    pub fn new(repeat: Option<i64>, interval: Option<i64>, command: Option<&'static str>, windows: Option<pty::Winszed>,) -> Result<Self> {
        let dynamic: Compositer = try!(Compositer::new());
        let shell: pty::Shell = try!(pty::Shell::new(repeat, interval, command, windows));
        let graphic: editeur::Graphic = try!(editeur::Graphic::new());
        let pid = shell.get_pid();

        let mut neko = Neko {
            display: Display::default(),
            dynamic: dynamic,
            shell: shell,
            graphic: graphic,
            line: io::Cursor::new(Vec::new()),
            pid: pid,
        };
        neko.call();
        Ok(neko)
    }

    /// The method `neko` runs a neko command for first level of shell.
    fn neko(&mut self, key: pty::Key, state: &mut pty::ShellState) {
        if key.is_enter().bitand(
            self.pid.eq(&self.shell.get_pid())
        ) {
            let message: Option<String> =
                match &self.line.get_ref()
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .to_string()
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .as_slice()[..] {
                &["neko", ref arguments..] => {
                    state.set_input_keyown('\u{3}');
                    match arguments {
                        &["install", ref repository] => Some(
                            format!("{:?}", self.dynamic.install(repository))
                        ),
                        &["mount", ref libraryname, ref priority] => Some(
                            format!("{:?}",
                                    self.dynamic.mount(
                                        libraryname,
                                        priority.parse::<i64>().ok()))
                        ),
                        &["mount", ref libraryname] => Some(
                            format!("{:?}", self.dynamic.mount(libraryname, None))
                        ),
                        &["unmount", ref libraryname] => Some(
                            format!("{:?}", self.dynamic.unmount(libraryname))
                        ),
                        &["uninstall", ref libraryname] => Some(
                            format!("{:?}", self.dynamic.uninstall(libraryname))
                        ),
                        &["update", ref libraryname] => Some(
                            format!("{:?}", self.dynamic.update(libraryname))
                        ),
                        _ => None,
                    }
                },
                _ => None,
            };
            if let Some(message) = message {
                self.dynamic.set_message(message)
            }
            self.line.get_mut().clear();
            self.line.set_position(0);
        }
    }

    /// The method `line` updates the current command line of proccess.
    fn line(&mut self, key: pty::Key) {
        let position: usize = self.line.position() as usize;
        match key {
            key if key.is_left() => {
                self.line.set_position(position.checked_sub(1).unwrap_or_default() as u64);
            },
            key if key.is_right() => {
                self.line.set_position(position.checked_add(1).unwrap_or_default() as u64);
            },
            key if key.is_start_heading() => {
                self.line.set_position(0);
            },
            key if key.is_enquiry() => {
                let position: usize = self.line.get_ref().len().checked_sub(1).unwrap_or_default();
                self.line.set_position(position as u64);
            },
            key if key.is_backspace() => {
                let position: usize = self.line.position() as usize;
                if position != 0 {
                    let position: usize = position-1;
                    if position < self.line.get_ref().len() {
                        self.line.get_mut().remove(position);
                        self.line.set_position(position as u64);
                    }
                }
            },
            key if key.is_enter() => {},
            key if key.is_c0()  => {
                self.line.get_mut().clear();
                self.line.set_position(0);
            },
            pty::Key::Str(line) => unsafe {
                let position: usize = self.line.position() as usize;
                let glyphs: String = String::from_utf8_unchecked(
                    line.iter()
                          .filter(|c: &&u8| c.eq(&&b'\0').not())
                          .map(|c: &u8| *c)
                          .collect::<Vec<u8>>()
                );

                if let Some(count) = glyphs.len().checked_add(position) {
                    self.line.get_mut().extend(glyphs.chars());
                    self.line.set_position(count as u64);
                }
            },
            _ => {
                if let Some(glyph) = key.is_utf8() {
                    self.line.get_mut().insert(position, glyph);
                    self.line.set_position(position.checked_add(1).unwrap_or_default() as u64);
                }
            },
        };
    }

    fn call(&mut self) {
        let lib: &LibraryState = self.dynamic.get_state();
 
        let screen: &pty::Display = self.shell.get_screen();
        if let Some(sprite) = self.graphic.explicite_emotion(
            lib.get_sheet(),
            lib.get_emotion(),
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


    /// The mutator method `set_window_size_with` redimentionnes the window
    /// with a argument size.
    pub fn set_window_size_with(&mut self, size: &pty::Winszed) {
        self.shell.set_window_size_with(size);
    }
}

impl Iterator for Neko {
    type Item = pty::ShellState;

    fn next(&mut self) -> Option<pty::ShellState> {
        self.shell.next().and_then(|mut shell| {
            if let Some(&(pid, _)) = shell.is_task() {
                self.pid = pid;
            }
            if let Some(key) = shell.is_input_keydown() {
                self.line(key);
                self.neko(key, &mut shell);
            }
            self.dynamic.call(&shell);
            self.call();
            Some(shell)
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
        let mut disp: String = String::new();

        self.get_screen()
            .into_iter()
            .all(|character: (&pty::Character)| {
                 disp.push_str(format!("{}", character).as_str());
                 true
            });
        write!(f, "{}", disp)
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
