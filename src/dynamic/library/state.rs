use std::mem;
use std::fmt;
use std::str;

use ::editeur;
use ::libc;

#[repr(C)]
#[derive(Copy)]
pub struct LibraryState {
    sheet: editeur::Sheet,
    emotions: [libc::c_uchar; editeur::SPEC_MAX_DRAW],
    draws: [[libc::c_uchar; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
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
            let mut draws: [[libc::c_uchar; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut emotions: [libc::c_uchar; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut message: [libc::c_uchar; 1024] = mem::uninitialized();

            draws.copy_from_slice(&self.draws);
            emotions.copy_from_slice(&self.emotions);
            message.copy_from_slice(&self.message);
            LibraryState {
                sheet: self.sheet,
                emotions: emotions,
                draws: draws,
                message: message,
                unmount: self.unmount,
            }
        }
    }
}

impl fmt::Debug for LibraryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "LibraryState {{ sheet: {}, emotion: {}, draws: [{}, {}, {}, {}, ...], message: {:?}, unmount: {} }}",
                   self.sheet,
                   str::from_utf8_unchecked(&self.emotions),
                   str::from_utf8_unchecked(&self.draws[0]),
                   str::from_utf8_unchecked(&self.draws[1]), 
                   str::from_utf8_unchecked(&self.draws[2]), 
                   str::from_utf8_unchecked(&self.draws[3]),
                   str::from_utf8_unchecked(&self.message),
                   self.unmount)
        }
    }
}

impl Default for LibraryState {
    fn default() -> Self {
        LibraryState {
            sheet: editeur::Sheet::default(),
            emotions: [b'\0'; editeur::SPEC_MAX_DRAW],
            draws: [[b'\0'; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
            message: [b'\0'; 1024],
            unmount: b'\0',
        }
    }
}
