pub mod err;

pub use self::err::{PositionError, Result};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Position {
  LotusHandsOnFloor,
  LyingOnSomething,
  None,
}

impl Position {
  pub fn new(content: &'static str) -> Result<Self> {
    match content {
      "LotusHandsOnFloor" => Ok(Position::LotusHandsOnFloor),
      "LyingOnSomething" => Ok(Position::LyingOnSomething),
      "None" => Ok(Position::None),
      _ => unimplemented!()
    }
  }
}

impl Default for Position {
  fn default() -> Position {
    Position::None
  }
}
