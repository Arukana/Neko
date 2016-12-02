use ::editeur;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Movement {
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

impl Movement {
    pub fn get_cartesian(&self, with: usize) -> (usize, usize) {
        match *self {
            Movement::UpperLeft => (0, 0),
            Movement::UpperMiddle => (with/2-editeur::SPEC_MAX_X/2, 0),
            Movement::UpperRight => (with-editeur::SPEC_MAX_X, 0),
            _ => unimplemented!(),
        }
    }
}
