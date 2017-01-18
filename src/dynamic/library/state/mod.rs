mod relative;
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
  persona: Persona,
  tooltip: Tooltip,
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
        &self.persona.sheet
    }

    pub fn get_tooltip(&self) -> &Tooltip
    { &self.tooltip }

    pub fn get_persona(&self) -> &Persona
    { &self.persona }

    pub fn get_position(&self) -> &Position {
        &self.persona.position
    }

    /// The function `get_emotion` returns a reference on a ffi argument
    /// of detailed emotion by draw.
    pub fn get_emotion(&self)
        -> &[[editeur::Tuple; editeur::SPEC_MAX_XY];
    editeur::SPEC_MAX_DRAW] {
        &self.persona.emotion
    }

    pub fn set_message(&mut self,
        message: String,
    ) {
        self.tooltip.set_message(message);
    }
}

impl Clone for LibraryState {
    fn clone(&self) -> Self {
        LibraryState {
            persona: self.persona,
            tooltip: self.tooltip,
            unmount: self.unmount,
            lock: self.lock,
        }
    }
}

impl fmt::Debug for LibraryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LibraryState {{ persona: {:?}, tooltip: {:?}, unmount: {}, lock: {:?} }}",
               self.persona,
               self.tooltip,
               self.unmount,
               self.lock.ne(&0),
        )
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        LibraryState {
            persona: Persona::default(),
            tooltip: Tooltip::default(),
            unmount: b'\0',
            lock: b'\0',
        }
    }
}
