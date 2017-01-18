mod position;
mod cardinal;

use std::ops::BitAnd;

use ::editeur;
use ::std;

pub use self::cardinal::Cardinal;
pub use self::position::Position;

#[repr(C)]
#[derive(Copy)]
pub struct Persona {
    pub sheet: editeur::Sheet,
    pub emotion: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
    pub position: Position,
}

impl PartialEq for Persona {
     fn eq(&self, other: &Persona) -> bool {
         self.sheet.eq(&other.sheet).bitand(
             self.emotion.iter()
                 .zip(other.emotion.iter())
                 .all(|(emotions, other_emotions):
                       (&[editeur::Tuple; editeur::SPEC_MAX_XY],
                        &[editeur::Tuple; editeur::SPEC_MAX_XY])| {
                     emotions.iter()
                             .zip(other_emotions.iter())
                             .all(|(emotion, other_emotion):
                                   (&editeur::Tuple,
                                    &editeur::Tuple)| {
                                 emotion.eq(other_emotion)
                             })
                 })
         )
     }   
}

impl Clone for Persona
{ fn clone(&self) -> Self
  { unsafe
    { let mut emotion: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = std::mem::uninitialized();
      emotion.copy_from_slice(&self.emotion);
      Persona
      { sheet: self.sheet,
        emotion: emotion,
        position: Position::default(), }}}}

impl std::fmt::Debug for Persona
{ fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
  { write!(f, "Persona {{ sheet: {}, emotion: [{:?}, {:?}, {:?}, {:?}, ...], position: {:?} }}", self.sheet, &self.emotion[0][..8], &self.emotion[1][..8], &self.emotion[2][..8], &self.emotion[3][..8], self.position) }}

impl Default for Persona
{ fn default() -> Self
  { let mut pos = Position::default();
    pos.cardinal = Cardinal::UpperRight;
    Persona
    { sheet: editeur::Sheet::Bust,
      emotion: [[editeur::Tuple::default(); editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
      position: pos, }}}
