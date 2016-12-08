use ::editeur;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Position {
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

impl Position {
    pub fn get_cartesian(&self, with: usize) -> (usize, usize) {
        match *self {
            Position::UpperLeft => (0, 0),
            Position::UpperMiddle => (with/2-editeur::SPEC_MAX_X/2, 0),
            Position::UpperRight => (with-editeur::SPEC_MAX_X, 0),
            _ => unimplemented!(),
        }
    }
}

impl Default for Position {
    fn default() -> Position {
        Position::LowerRight
    }
}
