#### TODO
Limits the glyph from `new` constructor between private unicode.

```rust
// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/Neko
//
// This file may not be copied, modified, or distributed
// except according to those terms.

mod err;

pub use self::err::{PartError, Result};

/// The enum `Part` is the list of texel type from a neko body.

#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum Part {
    EyeLeft(char),
    EyeRight(char),
    EarLeft(char),
    EarRight(char),
    Nose(char),
    Mouth(char),
    Neck(char),
}

impl Part {

    /// The constructor function `new` makes a texel part.

    pub fn new(limb: &str, glyph: char) -> Result<Self> {
        match limb {
            "EyeLeft" => Ok(Part::EyeLeft(glyph)),
            _ => Err(PartError::UnknownPart),
        }
    }
}
```