

use ::pty;
use std::fmt;
use std::mem;
use std::ops::{BitAnd, Index, RangeTo};

use super::relative::Relative;

#[repr(C)]
#[derive(Copy)]
pub struct Tooltip {
    cardinal: Relative,
    message: [pty::Character; 1024],
}

impl Tooltip {
    /// Return the height of the message, assuming all `\n`
    pub fn get_height(&self) -> usize {
        self.message.iter().filter(|&&nl| nl.is_enter()).count() + 1
    }

    /// Return the width of the message, assuming the characters between all `\n`
    pub fn get_width(&self) -> usize {
        self.message.split(|&nl| nl.is_enter()).fold(0, |acc, x| {
            if x.iter().find(|&x| x.is_null()).is_none() && acc < x.len() {
                x.len()
            } else if x.iter().find(|&&x| x.is_null()).is_some() {
                match x.iter().position(|&x| x.is_null()) {
                    Some(i) => if acc < i { i } else { acc },
                    None => acc,
                }
            } else {
                acc
            }
        })
    }

    pub fn get_cardinal(&self) -> &Relative {
        &self.cardinal
    }

    pub fn get_message(&self) -> &[pty::Character; 1024] {
        &self.message
    }

    pub fn set_message(&mut self, message: String) {
        self.message
            .iter_mut()
            .zip(message.chars())
            .all(|(mut_character, character): (&mut pty::Character, char)| {
                *mut_character = pty::Character::from(character);
                true
            });
        self.message
            .iter_mut()
            .skip(message.len())
            .all(|mut_character: &mut pty::Character| {
                mut_character.clear();
                true
            });
    }
}

impl Index<usize> for Tooltip {
    type Output = pty::Character;

    fn index(&self, count: usize) -> &pty::Character {
        &self.message[count]
    }
}

impl Index<RangeTo<usize>> for Tooltip {
    type Output = [pty::Character];

    fn index(&self, range: RangeTo<usize>) -> &[pty::Character] {
        &self.message[range]
    }
}

impl PartialEq for Tooltip {
    fn eq(&self, other: &Tooltip) -> bool {
        self.cardinal
            .eq(&other.cardinal)
            .bitand(self.message
                .iter()
                .zip(other.message.iter())
                .all(|(letter, other_letter)| letter.eq(&other_letter)))
    }
}

impl fmt::Debug for Tooltip {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Tooltip {{ cardinal: {:?}, message: {} }}",
               self.cardinal,
               self.message
                   .iter()
                   .take(1024)
                   .map(|character| character.get_glyph())
                   .collect::<String>())
    }
}

impl Clone for Tooltip {
    fn clone(&self) -> Self {
        unsafe {
            let mut message: [pty::Character; 1024] = mem::uninitialized();
            message.copy_from_slice(&self.message);
            Tooltip {
                cardinal: self.cardinal,
                message: message,
            }
        }
    }
}

impl Default for Tooltip {
    fn default() -> Self {
        Tooltip {
            cardinal: Relative::Left,
            message: [pty::Character::from('\0'); 1024],
        }
    }
}
