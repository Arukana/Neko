#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PosFromNeko
{ Top = 0,
  Bottom = 1,
  Right = 2,
  Left = 3, }

impl Default for PosFromNeko
{ fn default() -> Self
  { PosFromNeko::Right }}
