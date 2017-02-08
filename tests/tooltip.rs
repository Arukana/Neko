extern crate neko;

use neko::prelude::*;

#[test]
fn test_tooltip_eq_message() {
    let mut a = Tooltip::default();
    let b = Tooltip::default();

    assert_eq!(a, b);
    a.set_message("hi".to_string());
    assert_ne!(a, b);
}

#[test]
fn test_tooltip_eq_cardinal() {
    let mut a = Tooltip::default();
    let b = Tooltip::default();

    assert_eq!(a, b);
    a.set_cardinal(Relative::Top);
    assert_ne!(a, b);
}
