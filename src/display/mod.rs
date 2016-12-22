use ::editeur;

use ::pty;
use std::ops::{BitAnd, Rem, Not};

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
                     message: &[pty::Character; editeur::SPEC_MAX_Y *
                                                MESSAGE_WIDTH],
                     (start_x, start_y): (usize, usize)) {
        let (end_x, end_y): (usize, usize) = (start_x + editeur::SPEC_MAX_X,
                                              start_y + editeur::SPEC_MAX_Y);
        let width_term: usize = screen.get_window_size().get_col();
        let height_term: usize = screen.get_window_size().get_row();
        let mut draw_it = draw.into_iter();
        let mut text_it = message.into_iter();
        let mes_x: i64 = if start_y + MESSAGE_HEIGHT <= height_term {
            if end_x + MESSAGE_WIDTH + 1 <= width_term {
                (end_x + MESSAGE_WIDTH + 1) as i64
            } else if start_x > MESSAGE_WIDTH {
                (start_x - MESSAGE_WIDTH) as i64 - 1
            } else {
                -1
            }
        } else {
            -1
        };

        self.size = *screen.get_window_size();
        self.screen = screen.into_iter()
            .enumerate()
            .map(|(index, character): (usize, &pty::Character)|
                 index.checked_div(width_term).and_then(|y|
                    if mes_x.gt(&0).bitand(
                       (start_x..end_x).contains(index.rem(width_term)).bitand(
                       (start_y..end_y).contains(y))) {
                       draw_it.next().and_then(|&(_, texel)|
                           Some((index, pty::Character::from(texel.get_glyph()))))
                    } else {
                        None
                    }
                 ).unwrap_or_else(|| (index, *character)))
            .map(|(index, character): (usize, pty::Character)|
            { if mes_x > 0i64
              { index.checked_div(width_term).and_then(|y|
                { if mes_x > start_x as i64
                  { if {end_x + 1..(mes_x as usize)}.contains(index.rem(width_term)).bitand((start_y..end_y).contains(y))
                    { text_it.next().and_then(|&letter: &pty::Character|
                      { if letter.is_null().not()
                        { Some(letter) }
                        else
                        { None }}) }
                    else
                    { Some(character) }}
                  else
                  { if {(mes_x as usize)..start_x - 1}.contains(index.rem(width_term)).bitand((start_y..end_y).contains(y))
                    { text_it.next().and_then(|&letter: &pty::Character|
                      { if letter.is_null().not()
                        { Some(letter) }
                        else
                        { None }}) }
                    else
                    { Some(character) }}}).unwrap_or_else(|| character) }
              else
              { character }})
              .collect::<Vec<pty::Character>>();
    }
}

impl<'a> IntoIterator for &'a Display {
    type Item = &'a pty::Character;
    type IntoIter = ::std::slice::Iter<'a, pty::Character>;

    fn into_iter(self) -> Self::IntoIter {
        self.screen.as_slice().into_iter()
    }
}
