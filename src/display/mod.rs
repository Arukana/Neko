use std::ops::{BitAnd, Rem};

use ::pty;
use ::libc;
use ::editeur;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Display {
    screen: Vec<pty::Character>,
}

impl Display {
    pub fn with_draw(&mut self,
                     display: &pty::Display,
                     draw: &editeur::Draw,
                     message: &[libc::c_uchar; 1024],
                     (start_x, start_y): (usize, usize),
    ) {
        let (end_x, end_y): (usize, usize) =
            (start_x + editeur::SPEC_MAX_X, start_y + editeur::SPEC_MAX_Y);
        let with: usize = display.get_window_size().get_col();
        let mut draw_it = draw.into_iter();
        let mut text_it = message.into_iter();

        self.screen = display.into_iter()
            .enumerate()
            .map(|(index, character): (usize, &pty::Character)|
                 index.checked_div(with).and_then(|y|
                    if (start_x..end_x).contains(index.rem(with)).bitand(
                       (start_y..end_y).contains(y)) {
                       draw_it.next().and_then(|&(_, texel)|
                           Some((index, pty::Character::from(texel.get_glyph()))))
                    } else {
                        None
                    }
                 ).unwrap_or_else(|| (index, *character)))
            .map(|(index, character): (usize, pty::Character)|
                   if index > 40 {
                       text_it.next()
                           .and_then(|text: &u8|
                                     if text.ne(&b'\0') {
                                         Some(pty::Character::from(*text as char))
                                     } else {
                                         None
                                     })
                           .unwrap_or_else(|| character)

                   } else {
                       character
                   }
                )
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
