use std::mem;
use std::fmt;
use std::str;

use ::editeur;
use ::pty;
use ::libc;

pub const MESSAGE_WIDTH: usize = 16; // 16 + slide

use super::Position;

#[repr(C)]
#[derive(Copy)]
pub struct LibraryState {
    sheet: editeur::Sheet,
    emotion: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
    position: Position,
    message: [pty::Character; editeur::SPEC_MAX_Y * MESSAGE_WIDTH],
    unmount: libc::c_uchar,
}

impl LibraryState {
    pub fn is_unmounted(&self) -> bool {
        self.unmount.ne(&b'\0')
    }
 
    pub fn get_sheet(&self) -> &editeur::Sheet {
        &self.sheet
    }

    pub fn get_message(&self)
        -> &[pty::Character; editeur::SPEC_MAX_Y * MESSAGE_WIDTH] {
        &self.message
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    /// The function `get_emotion` returns a reference on a ffi argument
    /// of detailed emotion by draw.
    pub fn get_emotion(&self)
        -> &[[editeur::Tuple; editeur::SPEC_MAX_XY];
    editeur::SPEC_MAX_DRAW] {
        &self.emotion
    }

    pub fn set_message(&mut self,
        message: String,
    ) {
       self.message.iter_mut().zip(message.chars())
                   .all(|(mut_character,
                          character): (&mut pty::Character,
                                       char)| {
                        *mut_character = pty::Character::from(character);
                        true
                    });
    }
}

impl Clone for LibraryState {
    fn clone(&self) -> Self {
        unsafe {
            let mut emotion: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = mem::uninitialized();
            let mut message: [pty::Character; editeur::SPEC_MAX_Y * MESSAGE_WIDTH] = mem::uninitialized();

            emotion.copy_from_slice(&self.emotion);
            message.copy_from_slice(&self.message);
            LibraryState {
                sheet: self.sheet,
                emotion: emotion,
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
               &self.emotion[0][..8],
               &self.emotion[1][..8], 
               &self.emotion[2][..8], 
               &self.emotion[3][..8],
               self.message.iter().take(30).map(|character| character.get_glyph()).collect::<String>(),
               self.position,
               self.unmount)
    }
}

impl Default for LibraryState {
    fn default() -> Self {
      let mut message = [pty::Character::from('\0'); editeur::SPEC_MAX_Y * MESSAGE_WIDTH];
      let coucou = "Ca fait 10heuresque tu bosses!  Tu devrais push et arreter pour aujourd'hui!!";
      coucou.chars().enumerate().all(|(i, mes)|
      { message[i] = pty::Character::from(mes);
        true });
      LibraryState {
            sheet: editeur::Sheet::Bust,
            emotion: [[editeur::Tuple::default(); editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
            position: Position::default(),
            message: message,
            unmount: b'\0',
        }
    }
}
