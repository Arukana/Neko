use ::editeur;

use ::pty;
use dynamic::library::say::{Say, Relative};
use std::ops::{BitAnd, Rem, Not};

pub const MESSAGE_WIDTH: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Display
{ size: pty::Winszed,
  infobulle: Say,
  coord_neko: (usize, usize),
  draw: editeur::Draw,
  count: usize }

fn ultime_coordinates(size: pty::Winszed, mut coord_neko: (usize, usize), infobulle: Say) -> ((usize, usize), (usize, usize))
{ let width_message = infobulle.get_width();
  let height_message = infobulle.get_height();
  let row = size.get_row();
  let col = size.get_col();
  let coord_bulle: (usize, usize);
  match infobulle.cardinal
  { Relative::Top =>
      { if coord_neko.1 < height_message || coord_neko.1 + editeur::SPEC_MAX_Y + height_message >= row
        { coord_bulle = (coord_neko.0, 0);
          coord_neko = (coord_neko.0, coord_neko.1 + height_message); }
        else
        { coord_bulle = (coord_neko.0, coord_neko.1 - height_message); }}
    Relative::Bottom =>
      { if coord_neko.1 + editeur::SPEC_MAX_Y + height_message >= row
        { if editeur::SPEC_MAX_Y + height_message < row
          { coord_neko = (coord_neko.0, row - (editeur::SPEC_MAX_Y + height_message)); }
          else
          { coord_neko = (coord_neko.0, 0); }}
        coord_bulle = (coord_neko.0, coord_neko.1 + editeur::SPEC_MAX_Y); }
    Relative::Right =>
      { if coord_neko.0 + editeur::SPEC_MAX_X + width_message >= row
        { if editeur::SPEC_MAX_X + width_message < col
          { coord_neko = (col - (editeur::SPEC_MAX_X + width_message), coord_neko.1); }
          else
          { coord_neko = (0, coord_neko.1); }}
        coord_bulle = (coord_neko.0 + editeur::SPEC_MAX_X, coord_neko.1); }
    Relative::Left =>
      { if coord_neko.0 < width_message || coord_neko.0 + editeur::SPEC_MAX_X + width_message >= col
        { coord_bulle = (0, coord_neko.1);
          coord_neko = (coord_neko.0 + width_message, coord_neko.1); }
        else
        { coord_bulle = (coord_neko.0 - width_message, coord_neko.1); }}}
  (coord_bulle, coord_neko) }

impl Display
{ pub fn new(size: &pty::Winszed, infobulle: &Say, coord_neko: (usize, usize), sprite: &editeur::Draw) -> Self
  { Display
    { size: *size,
      infobulle: *infobulle,
      coord_neko: coord_neko,
      draw: sprite.clone(),
      count: 0 }}}

impl Iterator for Display
{ type Item = pty::Character;

  fn next(&mut self) -> Option<pty::Character>
  { if self.count >= self.size.get_col() * self.size.get_row()
    { None } else {
    self.count += 1; 
    let (coord_bulle, coord_neko): ((usize, usize), (usize, usize)) = ultime_coordinates(self.size, self.coord_neko, self.infobulle);
    let mut draw = self.draw.into_iter();
    if (coord_neko.1..(coord_neko.1 + editeur::SPEC_MAX_Y)).contains((self.count - 1) / self.size.get_col()) && (coord_neko.0..(coord_neko.0 + editeur::SPEC_MAX_X)).contains((self.count - 1) % self.size.get_col())
    { Some(pty::Character::from(draw.next().unwrap().1.get_glyph())) }
    else if (coord_bulle.1..(coord_bulle.1 + self.infobulle.get_height())).contains((self.count - 1) / self.size.get_col()) && (coord_bulle.0..(coord_bulle.0 + self.infobulle.get_width())).contains((self.count - 1) % self.size.get_col())
    { Some(self.infobulle.message[(((self.count - 1) / self.size.get_col()) - coord_bulle.1) + (((self.count - 1) % self.size.get_col()) - coord_bulle.0)]) }
    else
    { Some(pty::Character::from(' ')) }}}}
