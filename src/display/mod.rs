
use dynamic::library::say::{Say, Relative};
use dynamic::library::state::LibraryState;
use dynamic::library::personnage::Personnage;

use ::editeur;

use ::pty;
use std::ops::{BitAnd, BitOr, Rem, Not};

pub const MESSAGE_WIDTH: usize = 1024;

#[derive(Copy, Clone, Debug, Default)]
pub struct Display {
    size: pty::Winszed,
    coord_neko: (usize, usize),
    coord_bulle: (usize, usize),
    draw: editeur::Draw,
    count: usize,
    nl: (usize, usize),
    personage: Personnage,
    message: Say,
    padding: usize,
}

impl Display {
    pub fn from_window_size(size: &pty::Winszed) -> Self {
        let mut display: Display = Display::default();

        display.set_window_size(size);
        display
    }

    fn get_personage(&self) -> &Personnage {
        &self.personage
    }

    fn get_message(&self) -> &Say {
        &self.message
    }

    pub fn set_window_size(&mut self, size: &pty::Winszed) {
        self.size = *size;
    }

    /// The mutator method `set_state` updates the draw if
    /// the persona has changed of expression or sheet
    /// and overwrites the variable State of Neko's Display.
    pub fn set_state(&mut self, lib: &LibraryState, dictionary: &mut editeur::Graphic) {
 //       if self.get_message().ne(lib.get_infobulle()) {
            self.message = *lib.get_infobulle();
            self.nl = (self.message.get_width(), self.message.get_height());
 //       }
 //       if self.get_personage().ne(lib.get_personnage()) {
            self.coord_neko = lib.get_position().get_coordinate(&self.size);
            let (coord_bulle, coord_neko) = ultime_coordinates(self.size, self.coord_neko, self.message, (self.nl.0, self.nl.1));
            self.coord_bulle = coord_bulle;
            self.coord_neko = coord_neko;
            self.personage = *lib.get_personnage();
            self.draw =        dictionary.explicite_emotion(lib.get_sheet(), lib.get_emotion())
                          .and_then(|sprite| {
                               sprite.into_iter().next().and_then(|draw| {
                                   Some(*draw)
                               })
                           })
                          .unwrap_or_default();
 //       }
    }
}

impl Iterator for Display
{ type Item = pty::Character;

  fn next(&mut self) -> Option<pty::Character>
  { self.count += 1;
/*
println!("MESSAGE! COORD::{:?} | SIZE::{:?}", self.coord_bulle, self.nl);
println!("PAD::{} | CURS::({} ,{})", self.padding, (self.count - 1) % self.size.get_col(), ((self.count - 1) / self.size.get_col()));
*/
    let (coord_bulle, coord_neko) = (self.coord_bulle, self.coord_neko);
    let mut draw = self.draw.into_iter();
    if (coord_neko.1..(coord_neko.1 + editeur::SPEC_MAX_Y)).contains((self.count - 1) / self.size.get_col()) && (coord_neko.0..(coord_neko.0 + editeur::SPEC_MAX_X)).contains((self.count - 1) % self.size.get_col())
    { Some(pty::Character::from(draw.next().unwrap().1.get_glyph())) }

    else if (coord_bulle.1..(coord_bulle.1 + self.nl.1)).contains((self.count - 1) / self.size.get_col()) && (coord_bulle.0..(coord_bulle.0 + self.nl.0)).contains((self.count - 1) % self.size.get_col())
    { let k = ((((self.count - 1) / self.size.get_col()) - coord_bulle.1) * self.nl.0) + (((self.count - 1) % self.size.get_col()) - coord_bulle.0);
      if k < 1024
      { // println!("CAR::{:?}", self.message[k]);
        if self.padding == 0 && self.message[k].is_enter().not()
        { Some(self.message[k]) }
        else if self.padding == 0 && self.message[k].is_enter() && (self.count - 1) % self.size.get_col() > coord_bulle.0
        { self.padding = ((self.count - 1) / self.size.get_col()) + 1;
          Some(pty::Character::from(' ')) }
        else if self.padding < (self.count / self.size.get_col())
        { self.padding = 0;
          Some(pty::Character::from(' ')) }
        else
        { Some(pty::Character::from(' ')) }}
      else
      { Some(pty::Character::from(' ')) }}
    else
    { Some(pty::Character::from(' ')) }}}

/// Returns the cartesiane coordinate of infobulle and neko.
fn ultime_coordinates(
    size: pty::Winszed,
    mut coord_neko: (usize, usize),
    infobulle: Say,
    (width_message, height_message): (usize, usize),
) -> ((usize, usize), (usize, usize)) {
    let row = size.get_row();
    let col = size.get_col();
    let coord_bulle: (usize, usize);
    match infobulle.get_cardinal() {
        &Relative::Top => {
            if coord_neko.1 < height_message ||
               coord_neko.1 + editeur::SPEC_MAX_Y + height_message >= row {
                coord_bulle = (coord_neko.0, 0);
                coord_neko = (coord_neko.0, coord_neko.1 + height_message);
            } else {
                coord_bulle = (coord_neko.0, coord_neko.1 - height_message);
            }
        },
        &Relative::Bottom => {
            if coord_neko.1 + editeur::SPEC_MAX_Y + height_message >= row {
                if editeur::SPEC_MAX_Y + height_message < row {
                    coord_neko = (coord_neko.0,
                                  row - (editeur::SPEC_MAX_Y + height_message));
                } else {
                    coord_neko = (coord_neko.0, 0);
                }
            }
            coord_bulle = (coord_neko.0, coord_neko.1 + editeur::SPEC_MAX_Y);
        },
        &Relative::Right => {
            if coord_neko.0 + editeur::SPEC_MAX_X + width_message + 2 >= row {
                if editeur::SPEC_MAX_X + width_message + 2 < col {
                    coord_neko = (col - (editeur::SPEC_MAX_X + width_message + 2),
                                  coord_neko.1);
                } else {
                    coord_neko = (0, coord_neko.1);
                }
            }
            coord_bulle = (coord_neko.0 + editeur::SPEC_MAX_X + 2, coord_neko.1);
        },
        &Relative::Left => {
            if coord_neko.0 < width_message ||
                coord_neko.0 + editeur::SPEC_MAX_X + width_message >= col {
                  coord_bulle = (0, coord_neko.1);
                  coord_neko = (coord_neko.0 + width_message, coord_neko.1);
            } else {
                  coord_bulle = (coord_neko.0 - width_message, coord_neko.1);
            }
        },
    }
    (coord_bulle, coord_neko)
}
