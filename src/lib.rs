// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # neko
//!
//! This library contains the module `graphic` and `dynamic`.

#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]

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

#[macro_use]
extern crate itertools;
extern crate pty_proc;
extern crate editeur;
extern crate dylib;
extern crate git2;
extern crate toml;

#[macro_use]
/// The macros of crate.
mod macros;
/// The module `prelude` is for public.
pub mod prelude;
/// The module `dynamic` is the compositer of extern libraries.
pub mod dynamic;

mod err;


use dynamic::Compositer;
use editeur::Graphic;
use pty_proc::prelude::*;

pub use self::err::{NekoError, Result};
use std::fmt;
use std::slice;

use std::ops::{BitOr, BitAnd, Not};

/// The first directory.
const SPEC_ROOT: &'static str = ".neko";

/// Neko' size
const SPEC_NEKO_X_LEN: u8 = 10;
const SPEC_NEKO_Y_LEN: u8 = 5;
//const SPEC_NEKO_SIZE: u16 = (SPEC_NEKO_X_LEN as u16) * (SPEC_NEKO_Y_LEN as u16);

/// The module `neko` is the first interface level.
pub struct Neko {
    dynamic: Compositer,
    graphic: Graphic,
    shell: Shell,
   /* 
   /// `coord` les coordonnées de la Neko dans la matrice
   coord: (libc::size_t, libc::size_t),
   /// `neko_content` contenant les texels du sprite courant transmutés en u8
       // Transmute Exemple =>
       //   let c: char = 'a';
       //   let d: [u8; 4] = unsafe {
       //     std::mem::transmute::<char, [u8; 4]>(c)
       //   };
       //   println!("{} -> {:?}", c, d);
   neko_content: &[u8; SPEC_NEKO_X_LEN * SPEC_NEKO_Y_LEN * 4],
   /// `dessous_neko` la parcelle de matrice se trouvant sous la Neko lors de l'impression
   dessous_neko: &[Character; SPEC_NEKO_X_LEN * SPEC_NEKO_Y_LEN],
*/}

impl Neko {
    pub fn new(repeat: Option<i64>, interval: Option<i64>) -> Result<Self> {
        match (Shell::new(repeat, interval, None),
               Compositer::new(),
               Graphic::new()) {
            (Err(why), _, _) => Err(NekoError::Shell(why)),
            (_, Err(why), _) => Err(NekoError::Dynamic(why)),
            (_, _, Err(why)) => Err(NekoError::Graphic(why)),
            (Ok(shell), Ok(dynamic), Ok(graphic)) => {
                Ok(Neko {
                    dynamic: dynamic,
                    graphic: graphic,
                    shell: shell,
                  /*  coord(0, 0),
                    neko_content: [0; SPEC_NEKO_X_LEN * SPEC_NEKO_Y_LEN * 4],
                    dessous_neko: [0; SPEC_NEKO_SIZE],
               */ })
            }
        }
    }

    pub fn get_screen(&self) -> slice::Chunks<Character> {
        let display: &Display = self.shell.get_screen();
        let col: usize = display.get_window_size().get_col();

        display.into_iter()
            .as_slice()
            .chunks(col)
    }

/*
    pub fn display(&mut self) -> Result<>
    { let display: &mut Display = self.shell.get_mut_screen();
      let col: usize = display.get_window_size().get_col();
      let row: usize = display.get_window_size().get_row();
      if col.ge(&SPEC_NEKO_X_LEN).bitand(row.ge(&SPEC_NEKO_Y_LEN)).bitand()
      { let stock_cursor_coords: (libc::size_t, libc::size_t) = display.oob;
        self.neko_content = self.get_current_sprite().unwrap().1.get_current_draw().unwrap().into_iter().all(|&elem|
        { elem.get_glyph();
          true });

        // INSERT NEKO IN DISPLAY
        for i in SPEC_NEKO_Y_LEN
        { display.goto_coord(self.coord.0, self.coord.1 + i);
          // Stock ce qu'il y a à l'endroit de la Neko
          self.dessous_neko.write(&display.screen[self.coord.0 + ((self.coord.1 + i) * col) .. self.coord.0 + ((self.coord.1 + i) * col) + SPEC_NEKO_X_LEN])
          // Remplit screen avec la Neko
          display.write(&neko_content[i * SPEC_NEKO_X_LEN .. (i + 1) * SPEC_NEKO_X_LEN]) }
        display.oob = stock_cursor_coords;
        Ok() }
      else
      /// Terminal plus petit que la Neko ou
      /// la Neko est en dehors du Terminal
      { Err() }}
*/

}

impl Iterator for Neko {
    type Item = ShellState;

    fn next(&mut self) -> Option<ShellState> {
        if let Some(next) = self.shell.next() {
            self.dynamic.call(&next);
            Some(next)
        } else {
            None
        }
    }
}

impl fmt::Display for Neko {
    /// The function `fmt` formats the value using
    /// the given formatter.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.shell)
    }
}

impl fmt::Debug for Neko {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.dynamic, self.graphic)
    }
}
