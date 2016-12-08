use std::mem;
use std::fmt;
use std::str;

use ::editeur;
use ::libc;

use super::Position;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Tuple {
    part: editeur::Part,
    emotion: editeur::Emotion,
}

#[repr(C)]
#[derive(Copy)]
pub struct LibraryState {
    sheet: editeur::Sheet,
    emotions: [editeur::Emotion; editeur::SPEC_MAX_DRAW],
    draws: [[Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
    position: Position,
    cartesian: [libc::c_ushort; 2],
    message: [libc::c_uchar; 1024],
    unmount: libc::c_uchar,
}

impl LibraryState {
    pub fn is_unmounted(&self) -> bool {
        self.unmount.ne(&b'\0')
    }
}

impl Clone for LibraryState {
    fn clone(&self) -> Self {
        unsafe {
            let mut draws: [[Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut emotions: [editeur::Emotion; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut message: [libc::c_uchar; 1024] = mem::uninitialized();

            draws.copy_from_slice(&self.draws);
            emotions.copy_from_slice(&self.emotions);
            message.copy_from_slice(&self.message);
            LibraryState {
                sheet: self.sheet,
                emotions: emotions,
                draws: draws,
                message: message,
                position: self.position,
                cartesian: self.cartesian,
                unmount: self.unmount,
            }
        }
    }
}

impl fmt::Debug for LibraryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "LibraryState {{ sheet: {}, emotion: {:?}, draws: [{:?}, {:?}, {:?}, {:?}, ...],\
                                       message: {:?}, position: {:?}, cartesian: [{}; {}], unmount: {} }}",
                   self.sheet,
                   &self.emotions[..8],
                   &self.draws[0][..8],
                   &self.draws[1][..8], 
                   &self.draws[2][..8], 
                   &self.draws[3][..8],
                   str::from_utf8_unchecked(&self.message),
                   self.position,
                   self.cartesian[0],
                   self.cartesian[1],
                   self.unmount)
        }
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        LibraryState {
            sheet: editeur::Sheet::default(),
            emotions: [editeur::Emotion::default(); editeur::SPEC_MAX_DRAW],
            draws: [[Tuple::default(); editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
            position: Position::default(),
            cartesian: [0; 2],
            message: [b'\0'; 1024],
            unmount: b'\0',
        }
    }
}
