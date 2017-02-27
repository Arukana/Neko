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
extern crate editeur;
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
use dynamic::library::state::Relative;
use dynamic::library::state::persona::{Position, Cardinal};

pub use display::Display;

pub use editeur::prelude as graphic;
pub use pty_proc::prelude as pty;
pub use pty_proc::parent::Parent;

pub use self::err::{NekoError, Result};

/// The first directory.
pub const SPEC_ROOT: &'static str = editeur::SPEC_ROOT;
/// The default first directory.
pub const SPEC_ROOT_DEFAULT: &'static str = editeur::SPEC_ROOT_DEFAULT;

pub type PtyDisplay = pty::Display;

/// The module `neko` is the first interface level.
pub struct Neko <T> where T: Parent {
    dynamic: Compositer,
    /// Overload of Display interface from Pty.
    screen: Display,
    /// Interface of Pseudo terminal.
    shell: T,
    /// Interface on a Sprite partition.
    graphic: editeur::Graphic,
    /// The current pid.
    pid: libc::pid_t,
    /// The current Command.
    line: io::Cursor<Vec<char>>,
}

impl <T> Neko<T> where T: Parent {
    /// The constructor method `new` returns a Neko interface for a Shell, 
    /// a Compositer of dynamic libraries and a dictionnary of sprite.
    pub fn new(
        repeat: Option<i64>,
        interval: Option<i64>,
        command: Option<&str>,
        windows: Option<pty::Winszed>,
    ) -> Result<Neko<pty::Shell>> {
        let dynamic: Compositer = try!(Compositer::new());
        let shell: pty::Shell = try!(pty::Shell::new(repeat, interval, command, windows));
        let graphic: editeur::Graphic = try!(editeur::Graphic::new());
        let pid = shell.get_pid();
        let size: pty::Winszed = *shell.get_window_size();
        let mut neko = Neko {
            screen: Display::from_window_size(&size),
            dynamic: dynamic,
            shell: shell,
            graphic: graphic,
            line: io::Cursor::new(Vec::new()),
            pid: pid,
        };
        neko.call();
        Ok(neko)
    }

    /// The constructor method `from_shell` returns a Neko interface for a Shell, 
    /// a Compositer of dynamic libraries and a dictionnary of sprite.
    pub fn from_shell(
        shell: T,
    ) -> Result<Neko<T>> {
        let dynamic: Compositer = try!(Compositer::new());
        let graphic: editeur::Graphic = try!(editeur::Graphic::new());
        let pid = shell.get_pid();
        let size: pty::Winszed = *shell.get_window_size();
        let mut neko = Neko {
            screen: Display::from_window_size(&size),
            dynamic: dynamic,
            shell: shell,
            graphic: graphic,
            line: io::Cursor::new(Vec::new()),
            pid: pid,
        };
        neko.call();
        Ok(neko)
    }

    /// The method `call` updates the Neko's Display for a LibraryState
    /// and Graphic Dictionary.
    fn call(&mut self) {
        let lib: &LibraryState = self.dynamic.get_state();

        self.screen.set_state(lib, &mut self.graphic)
    }

    pub fn from_graphic(
        graphic: editeur::Graphic,
        repeat: Option<i64>,
        interval: Option<i64>,
        command: Option<&str>,
        windows: Option<pty::Winszed>,
    ) -> Result<Neko<pty::Shell>> {
        let dynamic: Compositer = try!(Compositer::new());
        let shell: pty::Shell = try!(pty::Shell::new(repeat, interval, command, windows));
        let pid = shell.get_pid();

        let neko = Neko {
            screen: Display::default(),
            dynamic: dynamic,
            shell: shell,
            graphic: graphic,
            line: io::Cursor::new(Vec::new()),
            pid: pid,
        };
        Ok(neko)
    }

    /// The method `neko` runs a neko command for first level of shell.
    #[allow(unused_must_use)]
    fn neko(&mut self, key: pty::Key, state: &mut pty::ShellState) {
        if key.is_enter().bitand(
            self.pid.eq(&self.shell.get_pid())
        ) {
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
                        &["debug"] => {
                            format_subneko!(self,
                                format!("{}\n{}",
                                    self.screen.get_persona(),
                                    self.screen.get_tooltip(),
                                ).as_bytes()
                            );
                        },
                        &["install", ref repository] => {
                            format_subneko!(self, repository, "install",
                                self.dynamic.install(repository)
                            );
                        },
                        &["uninstall", ref libraryname] => {
                            format_subneko!(self, libraryname, "uninstall",
                                self.dynamic.uninstall(libraryname)
                            );
                        },
                        &["mount", ref libraryname, ref priority] => {
                            format_subneko!(self, libraryname, "mount",
                                self.dynamic.mount(
                                    libraryname,
                                    priority.parse::<i64>().ok()
                                )
                            );
                        },
                        &["mount", ref libraryname] => {
                            format_subneko!(self, libraryname, "mount",
                                self.dynamic.mount(
                                    libraryname,
                                    None
                                )
                            );
                        },
                        &["unmount", ref libraryname] => {
                            format_subneko!(self, libraryname, "unmount",
                                self.dynamic.unmount(libraryname)
                            );
                        },
                        &["update", ref libraryname] => {
                            format_subneko!(self, libraryname, "update the library",
                                self.dynamic.update(libraryname)
                            );
                        },
                        &["persona", ref x, ref y] => {
                            match (x.parse::<u16>(), y.parse::<u16>()) {
                                (Ok(x), Ok(y)) => {
                                    self.dynamic.set_persona_position(
                                        Position::from([x, y])
                                    );
                                },
                                (Err(why), _) => {
                                    format_subneko_err!(
                                        self, x, "update the Persona's coordinate", why
                                    );
                                },
                                (_, Err(why)) => {
                                    format_subneko_err!(
                                        self, y, "update the Persona's coordinate", why
                                    );
                                },
                            }
                        },
                        &["persona", "UpperLeft"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::UpperLeft)
                            );
                        },
                        &["persona", "UpperMiddle"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::UpperMiddle)
                            );
                        },
                        &["persona", "UpperRight"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::UpperRight)
                            );
                        },
                        &["persona", "MiddleLeft"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::MiddleLeft)
                            );
                        },
                        &["persona", "MiddleCentral"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::MiddleCentral)
                            );
                        },
                        &["persona", "MiddleRight"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::MiddleRight)
                            );
                        },
                        &["persona", "LowerLeft"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::LowerLeft)
                            );
                        },
                        &["persona", "LowerMiddle"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::LowerMiddle)
                            );
                        },
                        &["persona", "LowerRight"] => {
                            self.dynamic.set_persona_position(
                                Position::from(Cardinal::LowerRight)
                            );
                        },
                        &["persona", ref sheetname] => {
                            match graphic::Sheet::new(sheetname) {
                                Ok(sheet) => {
                                    self.dynamic.set_persona_sheet(sheet);
                                },
                                Err(why) => {
                                    format_subneko_err!(
                                        self, sheetname, "update the Persona's sheet", why
                                    );
                                },
                            }
                        },
                        &["tooltip", "Top"] => {
                            self.dynamic.set_tooltip_cardinal(Relative::Top)
                        },
                        &["tooltip", "Bottom"] => {
                            self.dynamic.set_tooltip_cardinal(Relative::Bottom)
                        },
                        &["tooltip", "Right"] => {
                            self.dynamic.set_tooltip_cardinal(Relative::Right);
                        },
                        &["tooltip", "Left"] => {
                            self.dynamic.set_tooltip_cardinal(Relative::Left);
                        },
                        &["tooltip", ref text] => {
                            self.dynamic.set_tooltip_message(text.to_string());
                        },
                        _ => format_subneko!(self, b"The command's argument is unvalid."),
                    }
                },
                _ => {},
            };
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
                let position: usize = position.checked_add(1).unwrap_or_default();
                if position < self.line.get_ref().len() {
                    self.line.set_position(position as u64);
                }
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
                    if position.eq(&self.line.get_ref().len()) {
                        self.line.get_mut().insert(position, glyph);
                        self.line.set_position(position.checked_add(1).unwrap_or_default() as u64);
                    }
                }
            },
        };
    }

    /// The accessor method `get_screen` returns a reference on the Display interface.
    pub fn get_screen(&self) -> (&pty::Display, &Display) {
        (self.shell.get_screen(), &self.screen)
    }
}

impl <T> Parent for Neko<T> where T: Parent {

    /// The mutator method `write_screen` set a buffer to the display
    /// without needing to print it
    fn write_screen(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.shell.write_screen(buf)
    }

    /// The accessor method `get_pid` returns the pid from the master.
    fn get_pid(&self) -> libc::pid_t {
        self.shell.get_pid()
    }

    /// The accessor method `get_speudo` returns the master interface.
    fn get_speudo(&self) -> &pty::Master {
        self.shell.get_speudo()
    }

    /// The accessor method `get_screen` returns a reference on the Display interface.
    fn get_screen(&self) -> &pty::Display {
        self.shell.get_screen()
    }

    /// The accessor method `get_window_size` returns a reference on the window size of
    /// terminal.
    fn get_window_size(&self) -> &pty::Winszed {
        self.shell.get_window_size()
    }

    /// The mutator method `set_window_size` redimentionnes the window
    /// with a default size.
    fn set_window_size(&mut self) {
        self.shell.set_window_size()
    }

    /// The mutator method `set_window_size_with` redimentionnes the window
    /// with a argument size.
    fn set_window_size_with(&mut self, size: &pty::Winszed) {
        self.shell.set_window_size_with(size);
    }
}

impl <T> Iterator for Neko<T> where T: Parent {
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
            if let Some(ref size) = shell.is_resized() {
                self.screen.set_window_size(size);
                self.dynamic.resized(size);
            }
            self.dynamic.call(&shell);
            self.call();
            Some(shell)
        })
    }
}

impl <T> Write for Neko<T> where T: Parent {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.dynamic.get_state().is_locked().not() {
            self.shell.write(buf)
        } else {
            Ok(0)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.dynamic.get_state().is_locked().not() {
           self.shell.flush()
        } else {
            Ok(())
        }
    }
}

impl <T> fmt::Display for Neko<T> where T: Parent {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut disp: String = String::new();
            
        self.shell.get_screen()
            .into_iter()
            .zip(self.screen.into_iter())
            .all(|(pty_character, character)| {
                if character.is_null().not() {
                    disp.push_str(format!("{}", character).as_str());
                    true
                } else {
                    disp.push_str(format!("{}", pty_character).as_str());
                    true
                }
            });
         write!(f, "{}", disp)
    }
}

impl <T> fmt::Debug for Neko<T> where T: Parent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Neko {{ dynamic: {:?}, graphic: {:?}, display: {:?} }}",
               self.dynamic,
               self.graphic,
               self.screen)
    }
}

impl <T> Drop for Neko<T> where T: Parent {
    fn drop(&mut self) {
    }
}
