use std::mem;
use std::fmt;
use std::str;

use ::editeur;
use ::libc;

use super::Position;

#[repr(C)]
#[derive(Copy)]
pub struct LibraryState {
    sheet: editeur::Sheet,
    implicite: [editeur::Emotion; editeur::SPEC_MAX_DRAW],
    explicite: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
    position: Position,
    message: [libc::c_uchar; 1024],
    unmount: libc::c_uchar,
}

impl LibraryState {
    pub fn is_unmounted(&self) -> bool {
        self.unmount.ne(&b'\0')
    }
 
    pub fn get_sheet(&self) -> &editeur::Sheet {
        &self.sheet
    }

    pub fn get_message(&self) -> &[libc::c_uchar; 1024] {
        &self.message
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// The function `get_implicite` returns a reference on a ffi argument
    /// of generic emotion by draw.
    pub fn get_implicite(&self) -> &[editeur::Emotion; editeur::SPEC_MAX_DRAW] {
        &self.implicite
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
            let mut implicite: [editeur::Emotion; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut explicite: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut message: [libc::c_uchar; 1024] = mem::uninitialized();

            implicite.copy_from_slice(&self.implicite);
            explicite.copy_from_slice(&self.explicite);
            message.copy_from_slice(&self.message);
            LibraryState {
                sheet: self.sheet,
                implicite: implicite,
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
        unsafe {
            write!(f, "LibraryState {{ sheet: {}, emotion: {:?}, draws: [{:?}, {:?}, {:?}, {:?}, ...],\
                                       message: {:?}, position: {:?}, unmount: {} }}",
                   self.sheet,
                   &self.implicite[..8],
                   &self.explicite[0][..8],
                   &self.explicite[1][..8], 
                   &self.explicite[2][..8], 
                   &self.explicite[3][..8],
                   str::from_utf8_unchecked(&self.message),
                   self.position,
                   self.unmount)
        }
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        LibraryState {
            sheet: editeur::Sheet::Bust,
            implicite: [editeur::Emotion::default(); editeur::SPEC_MAX_DRAW],
            explicite: [[editeur::Tuple::default(); editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
            position: Position::default(),
            message: [b'\0'; 1024],
            unmount: b'\0',
        }
    }
}
