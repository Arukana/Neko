pub mod relative;

use std::ops::{BitAnd, Index, RangeTo};
use std::mem;
use std::fmt;

pub use self::relative::Relative;

use ::editeur;
use ::pty;

#[repr(C)]
#[derive(Copy)]
pub struct Say
{ cardinal: Relative,
  message: [pty::Character; 1024], }

impl Say
{ /// Return the height of the message, assuming all `\n`
  pub fn get_height(&self) -> usize
  { self.message.iter().filter(|&&nl| nl.is_enter()).count() }

  /// Return the width of the message, assuming the characters between all `\n`
  pub fn get_width(&self) -> usize
  { self.message.split(|&nl| nl.is_enter()).fold(0, |acc, x|
    { if x.iter().find(|&x| x.is_null()).is_none() && acc < x.len()
      { x.len() }
      else if x.iter().find(|&&x| x.is_null()).is_some()
      { match x.iter().position(|&x| x.is_null())
        { Some(i) =>
            { if acc < i
              { i }
              else
              { acc }},
          None => acc, }}
      else
      { acc }}) }
  
    pub fn get_cardinal(&self) -> &Relative {
        &self.cardinal
    }

    pub fn set_message(&mut self, message: String) {
        self.message.iter_mut().zip(message.chars())
                    .all(|(mut_character,
                          character): (&mut pty::Character,
                                       char)| {
                        *mut_character = pty::Character::from(character);
                        true
                    });
        self.message.iter_mut().skip(message.len())
                    .all(|mut_character: &mut pty::Character| {
                        mut_character.clear();
                        true
                    });
    }
}

impl Index<usize> for Say {
    type Output = pty::Character;

    fn index(&self, count: usize) -> &pty::Character {
        &self.message[count]
    }
}

impl Index<RangeTo<usize>> for Say {
    type Output = [pty::Character];

    fn index(&self, range: RangeTo<usize>) -> &[pty::Character] {
        &self.message[range]
    }
}

impl PartialEq for Say {
    fn eq(&self, other: &Say) -> bool {
        self.cardinal.eq(&other.cardinal)
            .bitand(self.message.iter()
                        .zip(other.message.iter())
                        .all(|(letter, other_letter)| {
                            letter.eq(&other_letter)
                        })
            )
    }
}

impl fmt::Debug for Say
{ fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  { write!(f, "Say {{ cardinal: {:?}, message: {} }}", self.cardinal, self.message.iter().take(1024).map(|character| character.get_glyph()).collect::<String>()) }}

impl Clone for Say
{ fn clone(&self) -> Self
  { unsafe
    { let mut message: [pty::Character; 1024] = mem::uninitialized();
      message.copy_from_slice(&self.message); 
      Say
      { cardinal: self.cardinal,
        message: message, }}}}

impl Default for Say
{ fn default() -> Self
  { let mut mes = [pty::Character::from('\0'); 1024];
    // TYPICAL TEST
    let tmp: [char; 22] = ['B', 'o', 'n', 'j', 'o', 'u', 'r', '\n', 'C', 'o', 'u', 'c', 'o', 'u', ' ', 't', 'r', 'e', 's', 'o', 'r', '\n'];
    for i in {0..22}
    { mes[i] = pty::Character::from(tmp[i]); }
Say
    { cardinal: Relative::Left, 
      message: mes }}}
      //message: [pty::Character::from('\0'); 1024], }}}
