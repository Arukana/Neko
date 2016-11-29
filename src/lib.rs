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
extern crate libc;

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
use std::io::Write;

pub use self::err::{NekoError, Result};
use std::fmt;
use std::slice;

use std::ops::{BitOr, BitAnd, Not};

/// The first directory.
const SPEC_ROOT: &'static str = ".neko";

/// Neko' size
const SPEC_NEKO_X_LEN: usize = 10;
const SPEC_NEKO_Y_LEN: usize = 5;
const SPEC_NEKO_SIZE: usize = SPEC_NEKO_X_LEN * SPEC_NEKO_Y_LEN;

/// The module `neko` is the first interface level.
pub struct Neko {
    dynamic: Compositer,
    graphic: Graphic,
    shell: Shell,
   /// `coord` les coordonnees de la Neko dans la matrice
   coord: (libc::size_t, libc::size_t),
   /// `neko_content` contenant les texels du sprite courant transmutes en u8
   neko_content: [u8; SPEC_NEKO_SIZE * 4],
   /// `dessous_neko` la parcelle de matrice se trouvant sous la Neko lors de l'impression
   dessous_neko: [Character; SPEC_NEKO_SIZE],
}

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
                    coord: (0, 0),
                    neko_content: [0; SPEC_NEKO_SIZE * 4],
                    dessous_neko: [Character::new(&[b' ']); SPEC_NEKO_SIZE],
                })
            }
        }
    }

    pub fn get_mut_shell(&mut self) -> &mut Shell
    { &mut self.shell }

    pub fn get_screen(&self) -> slice::Chunks<Character> {
        let display: &Display = self.shell.get_screen();
        let col: usize = display.get_window_size().get_col();

        display.into_iter()
            .as_slice()
            .chunks(col)
    }

    pub fn display_at(&mut self, coord: (usize, usize)) -> Result<()>
    { let col: usize = self.shell.get_screen().get_window_size().get_col();
      let row: usize = self.shell.get_screen().get_window_size().get_row();
      self.coord = coord;
      if col.ge(&SPEC_NEKO_X_LEN).bitand(row.ge(&SPEC_NEKO_Y_LEN)).bitand(coord.0.lt(&col)).bitand(coord.1.lt(&row))
      { let (x_stock, y_stock) = self.shell.get_screen().get_cursor_coords();
        let mut content = self.neko_content;
        let mut dessous: [Character; SPEC_NEKO_SIZE] = [Character::new(&[b' ']); SPEC_NEKO_SIZE];
        let (x_neko, y_neko) = self.coord;
        let size_x = 
        if x_neko + SPEC_NEKO_X_LEN < col
        { SPEC_NEKO_X_LEN }
        else
        { col - x_neko };
        let size_y = 
        if y_neko + SPEC_NEKO_Y_LEN < row
        { SPEC_NEKO_Y_LEN }
        else
        { row - y_neko };
        match self.graphic.get_current_sprite()
        { Some(sprite) =>
            { match sprite.1.get_current_draw()
              { Some(draw) =>
                  { let mut i = 0;
                    draw.into_iter().all(|&(_, mut elem)|
                    { let mut buffer: &mut [u8] = &mut content[i..];
                      unsafe { buffer.write(&(std::mem::transmute::<char, [u8; 4]>(elem.get_glyph()))).unwrap(); }
                      i += 4;
                      true }); },
                None => {}, }},
          None => {}, }

        {0..size_y}.all(|i|
        { self.shell.write_screen((format!("\x1B[{};{}H", y_neko + i + 1, x_neko + 1)).as_bytes());

        /*
          let coucou = self.shell.get_screen().into_iter().skip(((y_neko + i) * col) + x_neko).take(size_x)
          .map(|elem: (&Character)| *elem).collect::<Vec<Character>>();
*/
          let mut j = 0;
          print!("ELEM::");
          println!("ALL");
          self.shell.get_screen().into_iter().skip(((y_neko + i) * col) + x_neko).take(size_x)
          .map(|&elem|
          { dessous[j + (i * size_x)] = elem;
            print!("({}, {})::{:?} | ", i, j, elem);
            j += 1; }
            );
        println!("");
          self.shell.write_screen(&content[(i * 4) * size_x .. ((i + 1) * 4) * size_x]);
          true });
        self.shell.write_screen((format!("\x1B[{};{}H", y_stock + 1, x_stock + 1)).as_bytes());
        Ok(()) }
      else
      { //Err(NekoError::Size)
        Ok(()) }}

/*
        print!("CONTENT::");
        for i in {0..SPEC_NEKO_SIZE}
        { print!("{} ", content[i]); }
        println!("");
*/
// println!("FROM::{} | TO::{} | AT::({} ,{})", (i * 4) * SPEC_NEKO_X_LEN, ((i + 1) * 4) * SPEC_NEKO_X_LEN, x, y + i);
//println!("TO::({}, {}) | GET::{:?} | AT::{}", x, y + i, &content[(i * 4) * SPEC_NEKO_X_LEN .. ((i + 1) * 4) * SPEC_NEKO_X_LEN], ((i + 1) * 4) * SPEC_NEKO_X_LEN);
       // Transmute Exemple =>
       //   let c: char = 'a';
       //   let d: [u8; 4] = unsafe {
       //     std::mem::transmute::<char, [u8; 4]>(c)
       //   };
       //   println!("{} -> {:?}", c, d);

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
