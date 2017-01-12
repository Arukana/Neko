use std::fmt;

use super::Cardinal;

use ::libc;
use ::pty;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub cardinal: Cardinal,
    pub cartesian: [libc::c_ushort; 2],
}

impl Default for Position {
    fn default() -> Position {
        Position {
            cardinal: Cardinal::default(),
            cartesian: [0, 0],
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position {{ cardinal: {:?}, cartesian: {:?} }}", self.cardinal, self.cartesian)
    }
}

impl Position {
    pub fn get_coordinate(&self, size: &pty::Winszed) -> (usize, usize) {
        match self.cartesian {
            [0, 0] => self.cardinal.get_coordinate(size),
            [x, y] => (x as usize, y as usize),
        }
    }
}
