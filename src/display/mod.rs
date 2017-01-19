use dynamic::library::state::{LibraryState, Relative};
use dynamic::library::state::persona::Persona;
use dynamic::library::state::tooltip::Tooltip;

use ::editeur;

use ::pty;
use std::ops::Not;

pub const MESSAGE_WIDTH: usize = 1024;

#[derive(Copy, Clone, Debug, Default)]
pub struct Display {
    size: pty::Winszed,
    coord_neko: (usize, usize),
    coord_bulle: (usize, usize),
    draw: editeur::Draw,
    count: usize,
    nl: (usize, usize),
    persona: Persona,
    tooltip: Tooltip,
    padding: usize,
    cursor: usize,
}

impl Display {
    pub fn from_window_size(size: &pty::Winszed) -> Self {
        let mut display: Display = Display::default();

        display.set_window_size(size);
        display
    }

    fn get_persona(&self) -> &Persona {
        &self.persona
    }

    fn get_tooltipe(&self) -> &Tooltip {
        &self.tooltip
    }

    pub fn set_window_size(&mut self, size: &pty::Winszed) {
        self.size = *size;
    }

    /// The mutator method `set_state` updates the draw if
    /// the personna has changed of expression or sheet
    /// and overwrites the variable State of Neko's Display.
    pub fn set_state(&mut self,
                     lib: &LibraryState,
                     dictionary: &mut editeur::Graphic) {
        //       if self.get_tooltip().ne(lib.get_tooltip()) {
        self.tooltip = *lib.get_tooltip();
        self.nl = (self.tooltip.get_width() + 2, self.tooltip.get_height());
        //       }
        //       if self.get_persona().ne(lib.get_persona()) {
        self.coord_neko = lib.get_position().get_coordinate(&self.size);
        let (coord_bulle, coord_neko) = self.get_coordinates();
        self.coord_bulle = coord_bulle;
        self.coord_neko = coord_neko;
        self.persona = *lib.get_persona();
        self.draw =
            dictionary.explicite_emotion(lib.get_sheet(), lib.get_emotion())
                .and_then(|sprite| {
                    sprite.into_iter().next().and_then(|draw| Some(*draw))
                })
                .unwrap_or_default();
        //       }
    }

    /// Returns the cartesiane coordinate of tooltip and neko.
    fn get_coordinates(&self) -> ((usize, usize), (usize, usize)) {
        let row = self.size.get_row();
        let col = self.size.get_col();
        let coord_bulle: (usize, usize);
        let (width_tooltip, height_tooltip): (usize, usize) = (self.nl.0, self.nl.1);
        let mut coord_neko: (usize, usize) = self.coord_neko;

        match self.tooltip.get_cardinal() {
            &Relative::Top => {
                if coord_neko.1 < height_tooltip ||
                   coord_neko.1 + editeur::SPEC_MAX_Y + height_tooltip >= row {
                    coord_bulle = (coord_neko.0, 0);
                    coord_neko = (coord_neko.0, coord_neko.1 + height_tooltip);
                } else {
                    coord_bulle = (coord_neko.0, coord_neko.1 - height_tooltip);
                }
            }
            &Relative::Bottom => {
                if coord_neko.1 + editeur::SPEC_MAX_Y + height_tooltip >= row {
                    if editeur::SPEC_MAX_Y + height_tooltip < row {
                        coord_neko = (coord_neko.0,
                                      row -
                                      (editeur::SPEC_MAX_Y + height_tooltip));
                    } else {
                        coord_neko = (coord_neko.0, 0);
                    }
                }
                coord_bulle = (coord_neko.0, coord_neko.1 + editeur::SPEC_MAX_Y);
            }
            &Relative::Right => {
                if coord_neko.0 + editeur::SPEC_MAX_X + width_tooltip >= row {
                    if editeur::SPEC_MAX_X + width_tooltip < col {
                        coord_neko = (col -
                                      (editeur::SPEC_MAX_X + width_tooltip),
                                      coord_neko.1);
                    } else {
                        coord_neko = (0, coord_neko.1);
                    }
                }
                coord_bulle = (coord_neko.0 + editeur::SPEC_MAX_X + 2,
                               coord_neko.1);
            }
            &Relative::Left => {
                if coord_neko.0 < width_tooltip ||
                   editeur::SPEC_MAX_X + width_tooltip >= col {
                    coord_bulle = (0, coord_neko.1);
                    coord_neko = (width_tooltip + 1, coord_neko.1);
                } else {
                    coord_bulle = (coord_neko.0 - width_tooltip, coord_neko.1);
                }
            }
        }
        (coord_bulle, coord_neko)
    }
}

impl Iterator for Display {
    type Item = pty::Character;

    fn next(&mut self) -> Option<pty::Character> {
        self.count += 1;
        let (coord_bulle, coord_neko) = (self.coord_bulle, self.coord_neko);
        if self.padding < ((self.count - 1) / self.size.get_col()) + 1 {
            self.padding = 0;
        }
        if (coord_neko.1..(coord_neko.1 + editeur::SPEC_MAX_Y)).contains((self.count - 1) / self.size.get_col()) &&
           (coord_neko.0..(coord_neko.0 + editeur::SPEC_MAX_X)).contains((self.count - 1) % self.size.get_col()) {
            let (_, texel) = self.draw.next().unwrap();
            Some(pty::Character::from(texel.get_glyph()))
        } else if (coord_bulle.1..(coord_bulle.1 + self.nl.1)).contains((self.count - 1) / self.size.get_col()) &&
                  (coord_bulle.0..(coord_bulle.0 + self.nl.0)).contains((self.count - 1) % self.size.get_col()) {
            if self.cursor < 1024 {
                if self.padding == 0 && self.tooltip[self.cursor].is_enter().not() {
                    self.cursor += 1;
                    Some(self.tooltip[self.cursor - 1])
                } else {
                    if self.padding == 0 && self.tooltip[self.cursor].is_enter() {
                        self.padding = ((self.count - 1) / self.size.get_col()) + 1;
                        self.cursor += 1;
                    }
                    Some(pty::Character::from('\0'))
                }
            } else {
                Some(pty::Character::from('\0'))
            }
        } else {
            Some(pty::Character::from('\0'))
        }
    }
}
