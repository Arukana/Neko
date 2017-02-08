extern crate neko;

use neko::prelude::*;

#[test]
fn test_persona_eq_sheet() {
    let mut a = Persona::default();
    let b = Persona::default();

    assert_eq!(a, b);
    a.set_sheet(Sheet::BustHappy);
    assert_ne!(a, b);
}

#[test]
fn test_persona_eq_cardinal() {
    let mut a = Persona::default();
    let b = Persona::default();

    assert_eq!(a, b);
    a.set_position(Position::from(Cardinal::LowerLeft));
    assert_ne!(a, b);
}
