mod relative;
pub mod tooltip;
pub mod persona;

use std::fmt;
use std::str;

use ::graphic;
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

    pub fn get_sheet(&self) -> &graphic::Sheet {
        self.persona.get_sheet()
    }

    pub fn get_tooltip(&self) -> &Tooltip {
        &self.tooltip
    }

    pub fn get_persona(&self) -> &Persona
    { &self.persona }

    pub fn get_position(&self) -> &Position {
        self.persona.get_position()
    }

    /// The function `get_emotion` returns a reference on a ffi argument
    /// of detailed emotion by draw.
    pub fn get_emotion(&self)
        -> &[[graphic::Tuple; graphic::SPEC_MAX_XY];
    graphic::SPEC_MAX_DRAW] {
        self.persona.get_emotion()
    }

    pub fn set_tooltip_message(&mut self,
        message: String,
    ) {
        self.tooltip.set_message(message);
    }

    pub fn set_tooltip_cardinal(&mut self, cardinal: Relative) {
        self.tooltip.set_cardinal(cardinal);
    }

    pub fn set_persona_sheet(&mut self, sheet: graphic::Sheet) {
        self.persona.set_sheet(sheet);
    }

    pub fn set_persona_position(&mut self, position: Position) {
        self.persona.set_position(position);
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
