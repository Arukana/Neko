

use ::libc;
use ::pty;
use std::fmt;

use super::Cardinal;

#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position {
    cardinal: Cardinal,
    cartesian: [libc::c_ushort; 2],
}

impl Position {
    pub fn get_coordinate(&self, size: &pty::Winszed) -> (usize, usize) {
        match self.cartesian {
            [0, 0] => self.cardinal.get_coordinate(size),
            [x, y] => (x as usize, y as usize),
        }
    }

    pub fn set_cardinal(&mut self, cardinal: Cardinal) {
        self.cardinal = cardinal;
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Position {{ cardinal: {:?}, cartesian: {:?} }}",
               self.cardinal,
               self.cartesian)
    }
}

impl From<Cardinal> for Position {
    fn from(cardinal: Cardinal) -> Position {
        Position {
            cardinal: cardinal,
            cartesian: [0, 0],
        }
    }
}

impl From<[libc::c_ushort; 2]> for Position {
    fn from(cartesian: [libc::c_ushort; 2]) -> Position {
        Position {
            cardinal: Cardinal::default(),
            cartesian: cartesian,
        }
    }
}

impl Default for Position {
    fn default() -> Position {
        Position {
            cardinal: Cardinal::default(),
            cartesian: [0, 0],
        }
    }
}
