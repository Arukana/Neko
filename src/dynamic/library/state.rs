use std::mem;
use std::fmt;
use std::str;

use ::editeur;
use ::pty;
use ::libc;

use super::Position;

#[repr(C)]
#[derive(Copy)]
pub struct LibraryState {
    sheet: editeur::Sheet,
    explicite: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
    position: Position,
    message: [pty::Character; 1024],
    unmount: libc::c_uchar,
}

impl LibraryState {
    pub fn is_unmounted(&self) -> bool {
        self.unmount.ne(&b'\0')
    }
 
    pub fn get_sheet(&self) -> &editeur::Sheet {
        &self.sheet
    }

    pub fn get_message(&self) -> &[pty::Character; 1024] {
        &self.message
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// The function `get_explicite` returns a reference on a ffi argument
    /// of detailed emotion by draw.
    pub fn get_explicite(&self)
        -> &[[editeur::Tuple; editeur::SPEC_MAX_XY];
    editeur::SPEC_MAX_DRAW] {
        &self.explicite
    }


}

impl Clone for LibraryState {
    fn clone(&self) -> Self {
        unsafe {
            let mut explicite: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut message: [pty::Character; 1024] = mem::uninitialized();

            explicite.copy_from_slice(&self.explicite);
            message.copy_from_slice(&self.message);
            LibraryState {
                sheet: self.sheet,
                explicite: explicite,
                message: message,
                position: Position::default(),
                unmount: self.unmount,
            }
        }
    }
}

impl fmt::Debug for LibraryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LibraryState {{ sheet: {}, emotion: [{:?}, {:?}, {:?}, {:?}, ...],\
                                   message: {:?}, position: {:?}, unmount: {} }}",
               self.sheet,
               &self.explicite[0][..8],
               &self.explicite[1][..8], 
               &self.explicite[2][..8], 
               &self.explicite[3][..8],
               self.message.iter().take(30).map(|character| character.get_glyph()).collect::<String>(),
               self.position,
               self.unmount)
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        LibraryState {
            sheet: editeur::Sheet::Bust,
            explicite: [[editeur::Tuple::default(); editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
            position: Position::default(),
            message: [pty::Character::default(); 1024],
            unmount: b'\0',
        }
    }
}
