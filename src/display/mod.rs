use std::ops::{BitAnd, Rem};

use ::pty;
use ::libc;
use ::editeur;

pub const MESSAGE_WIDTH: usize = 16; // 16 + slide
pub const MESSAGE_HEIGHT: usize = editeur::SPEC_MAX_Y;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Display {
    screen: Vec<pty::Character>,
    size: pty::Winszed,
}

impl Display {

    /// The accessor `get_window_size` returns the window size interface.
    pub fn get_window_size(&self) -> &pty::Winszed {
        &self.size
    }

    pub fn with_draw(&mut self,
                     screen: &pty::Display,
                     draw: &editeur::Draw,
                     message: &[libc::c_uchar; editeur::SPEC_MAX_Y * MESSAGE_WIDTH],
                     (start_x, start_y): (usize, usize),
    ) {
        let (end_x, end_y): (usize, usize) =
            (start_x + editeur::SPEC_MAX_X, start_y + editeur::SPEC_MAX_Y);
        let width_term: usize = screen.get_window_size().get_col();
        let height_term: usize = screen.get_window_size().get_row();
        let mut draw_it = draw.into_iter();
        let mut text_it = message.into_iter();
        let mes_x =
        if start_y + MESSAGE_HEIGHT <= height_term
        { if end_x + MESSAGE_WIDTH <= width_term
          { end_x + MESSAGE_WIDTH }
          else if start_x >= MESSAGE_WIDTH
          { start_x - MESSAGE_WIDTH }
          else
          { -1 }}
        else
        { -1 };

        self.size = *screen.get_window_size();
        self.screen = screen.into_iter()
            .enumerate()
            .map(|(index, character): (usize, &pty::Character)|
                 index.checked_div(width_term).and_then(|y|
                    if (start_x..end_x).contains(index.rem(width_term)).bitand(
                       (start_y..end_y).contains(y)) {
                       draw_it.next().and_then(|&(_, texel)|
                           Some((index, pty::Character::from(texel.get_glyph()))))
                    } else {
                        None
                    }
                 ).unwrap_or_else(|| (index, *character)))
            .map(|(index, character): (usize, pty::Character)|
            { if mes_x > 0
              { index.checked_div(width_term).and_then(|y|
                { if (start_x..end_x).contains(index.rem(width_term)).bitand((start_y..end_y).contains(y))
                  { text_it.next().and_then(|&letter|
                    { if letter != b'\0'
                      { Some((index, pty::Character::from(letter as char))) }
                      else
                      { None }}) }}).unwrap_or_else(|| (index, character)) }
              else
              { character }}).collect::<Vec<pty::Character>>();
    }
}
                   /*end_x + (end_y * size.get_col())*/

impl<'a> IntoIterator for &'a Display {
    type Item = &'a pty::Character;
    type IntoIter = ::std::slice::Iter<'a, pty::Character>;

    fn into_iter(self) -> Self::IntoIter {
        self.screen.as_slice().into_iter()
    }
}
