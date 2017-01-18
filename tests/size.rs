extern crate neko;

use neko::prelude::*;

use std::mem;

#[test]
fn test_size_library_state() {
    assert_eq!(mem::size_of::<LibraryState>(), 18708);
    assert_eq!(mem::size_of::<Position>(), 8);
    assert_eq!(mem::size_of::<Tooltip>(), 12292);
    assert_eq!(mem::size_of::<Persona>(), 6412);
    assert_eq!(mem::size_of::<Sheet>(), 4);
    assert_eq!(mem::size_of::<Tuple>(), 8);
    assert_eq!(mem::size_of::<[[Tuple; SPEC_MAX_DRAW]; SPEC_MAX_XY]>(), 6400);
    assert_eq!(mem::size_of::<Relative>(), 4);
    assert_eq!(mem::size_of::<Character>(), 12);
    assert_eq!(mem::size_of::<[Character; 1024]>(), 12288);
}
