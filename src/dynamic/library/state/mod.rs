pub mod relative;
pub mod tooltip;
pub mod persona;

use std::fmt;
use std::str;

use ::editeur;
use ::libc;

use self::tooltip::Tooltip;
use self::persona::{Persona, Position};
pub use self::relative::Relative;

#[repr(C)]
#[derive(Copy)]
pub struct LibraryState {
  neko: Persona,
  message: Tooltip,
  unmount: libc::c_uchar,
  lock: libc::c_uchar,
}

impl LibraryState {
    pub fn is_unmounted(&self) -> bool {
        self.unmount.ne(&b'\0')
    }

    pub fn is_locked(&self) -> bool {
        self.lock.ne(&b'\0')
    }
 
    pub fn get_sheet(&self) -> &editeur::Sheet {
        &self.neko.sheet
    }

    pub fn get_message(&self) -> &Tooltip
    { &self.message }

    pub fn get_personnage(&self) -> &Persona
    { &self.neko }

    pub fn get_position(&self) -> &Position {
        &self.neko.position
    }

    /// The function `get_emotion` returns a reference on a ffi argument
    /// of detailed emotion by draw.
    pub fn get_emotion(&self)
        -> &[[editeur::Tuple; editeur::SPEC_MAX_XY];
    editeur::SPEC_MAX_DRAW] {
        &self.neko.emotion
    }

    pub fn set_message(&mut self,
        message: String,
    ) {
        self.message.set_message(message);
    }
}

impl Clone for LibraryState {
    fn clone(&self) -> Self {
        LibraryState {
            neko: self.neko,
            message: self.message,
            unmount: self.unmount,
            lock: self.lock,
        }
    }
}

impl fmt::Debug for LibraryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LibraryState {{ neko: {:?}, message: {:?}, unmount: {}, lock: {:?} }}",
               self.neko,
               self.message,
               self.unmount,
               self.lock.ne(&0),
        )
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        LibraryState {
            neko: Persona::default(),
            message: Tooltip::default(),
            unmount: b'\0',
            lock: b'\0',
        }
    }
}
