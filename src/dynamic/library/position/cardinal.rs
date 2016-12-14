use ::editeur;
use ::pty;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Cardinal {
    UpperLeft = 0,
    UpperMiddle = 1,
    UpperRight = 2,
    MiddleLeft = 3,
    MiddleCentral = 4,
    MiddleRight = 5,
    LowerLeft = 6,
    LowerMiddle = 7,
    LowerRight = 8,
}

impl Cardinal {
    pub fn get_coordinate(&self, size: &pty::Winszed) -> (usize, usize) {
        let with: usize = size.get_col();
        let height: usize = size.get_row();
        match *self {
           Cardinal::UpperLeft => (0, 0),
            Cardinal::UpperMiddle => (with/2-editeur::SPEC_MAX_X/2, 0),
            Cardinal::UpperRight => (with-editeur::SPEC_MAX_X, 0),
            Cardinal::MiddleLeft => (0, height/2-editeur::SPEC_MAX_X/2),
            Cardinal::MiddleCentral => (with/2-editeur::SPEC_MAX_X/2, height/2-editeur::SPEC_MAX_X/2),
            Cardinal::MiddleRight => (with-editeur::SPEC_MAX_X, height/2-editeur::SPEC_MAX_X/2),
            Cardinal::LowerLeft => (0, height-editeur::SPEC_MAX_X),
            Cardinal::LowerMiddle => (with/2-editeur::SPEC_MAX_X/2, height-editeur::SPEC_MAX_Y),
            Cardinal::LowerRight => (with-editeur::SPEC_MAX_X, height-editeur::SPEC_MAX_Y),
        }
    }
}

impl Default for Cardinal {
    fn default() -> Cardinal {
        Cardinal::LowerRight
    }
}


