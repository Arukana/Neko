extern crate neko;

#[cfg(feature = "compositer_ffi")]
use neko::prelude::*;

#[cfg(feature = "compositer_ffi")]
use std::process::Command;

#[test]
#[ignore]
#[cfg(feature = "compositer_ffi")]
fn test_compositer_ffi() {
    {
        let compositer: Compositer = Compositer::new().unwrap();

        assert_eq!(
            &compositer.get_state().get_tooltip()[..6].iter().map(|c| c.get_glyph()).collect::<String>(),
            &"start\0".to_string()
        );
    }
}
