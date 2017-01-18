#[repr(u32)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Relative {
    Top = 0,
    Bottom = 1,
    Right = 2,
    Left = 3,
}

impl Default for Relative {
    fn default() -> Self {
        Relative::Right
    }
}
