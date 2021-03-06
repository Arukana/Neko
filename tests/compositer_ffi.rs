extern crate neko;

#[cfg(feature = "compositer_ffi")]
use neko::prelude::*;

#[test]
#[cfg(feature = "compositer_ffi")]
fn test_compositer_ffi() {
    {
        let mut compositer: Compositer = Compositer::new().unwrap();
        assert_eq!(
            compositer.get_state().get_persona().sheet,
	        Sheet::None
        );
        assert_eq!(
            &compositer.get_state().get_tooltip()[..6].iter().map(|c| c.get_glyph()).collect::<String>(),
            &"\0\0\0\0\0\0".to_string()
        );

        compositer.install(
            "https://github.com/Arukana/libnya.git"
        );
        assert_eq!(
            compositer.get_state().get_persona().sheet,
	        Sheet::Bust
        );
        assert_eq!(
            &compositer.get_state().get_tooltip()[..6].iter().map(|c| c.get_glyph()).collect::<String>(),
            &"start\0".to_string()
        );
    }
}
