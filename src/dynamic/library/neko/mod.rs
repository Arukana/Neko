pub mod cardinal;
pub mod position;

use ::editeur;
use ::pty;
use ::std;

pub use self::cardinal::Cardinal;
pub use self::position::Position;

#[repr(C)]
#[derive(Copy)]
pub struct NekoContent
{ pub sheet: editeur::Sheet,
  pub emotion: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
  pub position: Position, }

impl Clone for NekoContent
{ fn clone(&self) -> Self
  { unsafe
    { let mut emotion: [[editeur::Tuple; editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW] = std::mem::uninitialized();
      emotion.copy_from_slice(&self.emotion);
      NekoContent
      { sheet: self.sheet,
        emotion: emotion,
        position: Position::default(), }}}}

impl std::fmt::Debug for NekoContent
{ fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
  { write!(f, "NekoContent {{ sheet: {}, emotion: [{:?}, {:?}, {:?}, {:?}, ...], position: {:?} }}", self.sheet, &self.emotion[0][..8], &self.emotion[1][..8], &self.emotion[2][..8], &self.emotion[3][..8], self.position) }}

impl Default for NekoContent
{ fn default() -> Self
  { NekoContent
    { sheet: editeur::Sheet::Bust,
      emotion: [[editeur::Tuple::default(); editeur::SPEC_MAX_XY]; editeur::SPEC_MAX_DRAW],
      position: Position::default(), }}}

