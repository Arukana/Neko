use ::editeur;

use ::pty;
use std::ops::{BitAnd, Rem, Not};

pub const MESSAGE_WIDTH: usize = 1024;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Display
{ size: Winszed,
  infobulle: Say,
  coord_neko: (usize, usize),
  draw: editeur::Draw,
  count: usize } 

impl Display
{ fn new(size: Winszed, infobulle: Say, coord_neko: (usize, usize), sprite: Sprite) -> Self
  { Display
    { size: size,
      infobulle: infobulle,
      coord_neko: coord_neko,
      draw: sprite,
      count: 0 }}}

impl Iterator for Display
{ type Item = pty::Character;

  fn next(&mut self) -> Option<pty::Character>
  { if self.count >= self.size.get_col() * self.size.get_row()
    { self.count = 0; }
    self.count += 1; 
    let coord_bulle: (usize, usize) = self.infobulle.position.get_coordinates(self.size);
    let draw = self.draw.into_iter().next().unwrap();
    if (coord_neko.1..(coord_neko.1 + editeur::SPEC_MAX_Y)).contains((self.count - 1) / self.size.get_col()) && (coord_neko.0..(coord_neko.0 + editeur::SPEC_MAX_X)).contains((self.count - 1) % self.size.get_col())
    { Some(pty::Character::from(draw.next().unwrap())) }
    else if (coord_bulle.1..(coord_bulle.1 + self.infobulle.get_height())).contains((self.count - 1) / self.size.get_col()) && (coord_bulle.0..(coord_bulle.0 + self.infobulle.get_width())).contains((self.count - 1) % self.size.get_col())
    { Some(self.infobulle.message[(((self.count - 1) / self.size.get_col()) - coord_bulle.1) + (((self.count - 1) % self.size.get_col()) - coord_bulle.0)]) }
    else
    { Some(pty::Character::from(' ')) }}


/*
        if self.count == 10 {
            self.count = 0;
        } else {
            self.count += 1;
        }
        if self.count < 5 {
            Some(' ')
        } else {
            Some ('a')
        }
    }}
*/
