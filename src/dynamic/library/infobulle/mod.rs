pub mod cardinal;

pub use self::cardinal::PosFromNeko;

use ::editeur;
use ::pty;
use ::std;

#[repr(C)]
#[derive(Copy)]
pub struct InfoBulle
{ pub cardinal: PosFromNeko,
  pub message: [pty::Character; 1024], }

impl InfoBulle
{ pub fn get_height(&self) -> usize
  { self.message.iter().filter(|&&nl| nl.is_enter()).count() }

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
      { acc }}) }}

impl std::fmt::Debug for InfoBulle
{ fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
  { write!(f, "InfoBulle {{ cardinal: {:?}, message: {} }}", self.cardinal, self.message.iter().take(1024).map(|character| character.get_glyph()).collect::<String>()) }}

impl Clone for InfoBulle
{ fn clone(&self) -> Self
  { unsafe
    { let mut message: [pty::Character; 1024] = std::mem::uninitialized();
      message.copy_from_slice(&self.message); 
      InfoBulle
      { cardinal: self.cardinal,
        message: message, }}}}

impl Default for InfoBulle
{ fn default() -> Self
  { InfoBulle
    { cardinal: PosFromNeko::default(), 
      message: [pty::Character::from('\0'); 1024], }}}
