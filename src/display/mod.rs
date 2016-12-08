use std::ops::{BitAnd, Rem};

use ::pty;
use ::editeur;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Display {
    screen: Vec<pty::Character>,
    /// Col, in characters.
    size: pty::Winszed,
    /// Start at Row Point.
    start_x: usize,
    /// Start at Col Point.
    start_y: usize,
}

impl Display {

    pub fn get_window_size(&self) -> &pty::Winszed {
        &self.size
    }

    pub fn with_draw(
        &mut self, screen: &pty::Display, draw: &editeur::Draw,
    ) {
        let (end_x, end_y): (usize, usize) =
            (self.start_x + editeur::SPEC_MAX_X, self.start_y + editeur::SPEC_MAX_Y);
        let with: usize = self.size.get_col();
        let mut it = draw.into_iter();

        self.screen = screen.into_iter()
            .enumerate()
            .map(|(index, character): (usize, &pty::Character)|
                 index.checked_div(with).and_then(|y|
                    if (self.start_x..end_x).contains(index.rem(with)).bitand(
                       (self.start_y..end_y).contains(y)) {
                       it.next().and_then(|&(_, texel)|
                           Some(pty::Character::from(texel.get_glyph())))
                    } else {
                        None
                    }
                 ).unwrap_or_else(|| *character))
            .collect::<Vec<pty::Character>>();
    }

    pub fn set_window_size(&mut self, size: &pty::Winszed) {
        self.size = *size;
    }

    pub fn set_start(&mut self, x: usize, y: usize) {
        self.start_x = x;
        self.start_y = y;
    }
}



impl<'a> IntoIterator for &'a Display {
    type Item = &'a pty::Character;
    type IntoIter = ::std::slice::Iter<'a, pty::Character>;

    fn into_iter(self) -> Self::IntoIter {
        self.screen.as_slice().into_iter()
    }
}
