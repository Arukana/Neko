use ::editeur;

use ::pty;
use std::ops::{BitAnd, Rem, Not};

pub const MESSAGE_WIDTH: usize = 1024;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Display
{ size: Winsized,
  neko: Personnage,
  infobulle: Say,
  count: usize } 

impl Display
{ /// The accessor `get_window_size` returns the window size interface.
  pub fn get_window_size(&self) -> &pty::Winszed
  { &self.size }
  
  /// The accessor `set_window_size` set terminal size to `new_size` valie
  pub fn set_window_size(&mut self, new_size: &pty::Winszed)
  { self.size = new_size; }}

impl<'a> Iterator for Display
{ type Item = pty::Character;

  fn next(&mut self) -> Option<pty::Character>
  { if self.size.get_col() > 0 && self.size.get_row() > 0
    { if self.count >= self.size.get_col() * self.size.get_row()
      { self.count = 0; }
      self.count += 1; 
      let coord_neko: (usize, usize) = self.neko.position.get_coordinates(self.size);
      let coord_bulle: (usize, usize) = self.infobulle.position.get_coordinates(self.size);
      if {coord_neko.1..(coord_neko.1 + editeur::SPEC_MAX_Y)}.contains((self.count - 1) / self.size.get_col()) && {coord_neko.0..(coord_neko.0 + editeur::SPEC_MAX_X)}.contains((self.count - 1) % self.size.get_col())
      { /*Some(pty::Character::from(draw[]))*/ None }
      else if {coord_bulle.1..(coord_bulle.1 + self.infobulle.get_height())}.contains((self.count - 1) / self.size.get_col()) && {coord_bulle.0..(coord_bulle.0 + self.infobulle.get_width())}.contains((self.count - 1) % self.size.get_col())
      { Some(self.infobulle.message[(((self.count - 1) / self.size.get_col()) - coord_bulle.1) + (((self.count - 1) % self.size.get_col()) - coord_bulle.0)]) }
      else
      { Some(pty::Character::from(' ')) }}
    else
    { None }}}


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
