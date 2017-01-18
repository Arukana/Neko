extern crate neko;

use neko::prelude::*;

use self::std::path::PathBuf;
use self::std::env;

#[test]
#[cfg(feature = "compositer_command")]
fn test_compositer_command() {
    {
        env::set_var("NEKO_PATH", "/tmp/arukana1/.neko");
        let compositer: Compositer = Compositer::new().unwrap();

        assert_eq!(compositer.get_git().ok(),
                Some(PathBuf::from("/tmp/arukana1/.neko/git"))
        );
    }
    {
        env::set_var("NEKO_PATH", "/tmp/arukana2/.neko");
        let compositer: Compositer = Compositer::new().unwrap();

        assert_eq!(compositer.get_lib().ok(),
                Some(PathBuf::from("/tmp/arukana2/.neko/lib"))
        );
    }
    {
        env::set_var("NEKO_PATH", "/tmp/arukana3/.neko");
        let mut compositer: Compositer = Compositer::new().unwrap();

        assert_eq!(compositer.install(
            "https://github.com/Arukana/libnya.git"
        ).err(), None);
        assert_eq!(compositer.mount(
            &"arukana@libnya", None
        ).err(), None);
        assert_eq!(compositer.mount(
            &"arukana@libnya", None
        ).err(), None);
        assert_eq!(compositer.update(
            "arukana@libnya"
        ).err(), None);
        assert_eq!(compositer.install(
            "https://github.com/Arukana/libnya.git"
        ).err(), Some(CompositerError::InstallExists));
        assert_eq!(compositer.unmount(
            "arukana@libnya"
        ).err(), None);
        assert_eq!(compositer.unmount(
            "arukana@libnya"
        ).err(), Some(CompositerError::UnmountPosition));
        assert_eq!(compositer.uninstall(
            &"arukana@libnya"
        ).err(), None);
        assert_eq!(compositer.unmount(
            "arukana@libnya"
        ).err(), Some(CompositerError::UnmountPosition));
    }
    {
        env::set_var("NEKO_PATH", "/tmp/arukana4/.neko");
        let mut compositer: Compositer = Compositer::new().unwrap();

        assert_eq!(compositer.install(
            "https://github.com/Arukana/libnya.git"
        ).err(), None);
        assert_eq!(compositer.uninstall(
            &"arukana@libnya"
        ).err(), None);
        assert_eq!(compositer.uninstall(
            &"arukana@libnya"
        ).ok(), None);
    }
}
