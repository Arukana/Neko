#![feature(test)]

extern crate test;
extern crate neko;

use std::ops::Not;

use neko::prelude::*;

#[bench]
fn bench_screen_pty(b: &mut test::Bencher) {
    b.iter(|| {
        let mut disp: String = String::new();

        //let screen: Display = Display::default();
        let pty_screen: PtyDisplay = PtyDisplay::default();

        pty_screen.into_iter()
            .all(|character: (&Character)| {
                 disp.push_str(format!("{}", character).as_str());
                 true
            });
    });
}

#[bench]
fn bench_screen_neko(b: &mut test::Bencher) {
    b.iter(|| {
        let mut disp: String = String::new();

        let screen: Display = Display::default();
        //let pty_screen: PtyDisplay = PtyDisplay::default();

        screen.into_iter()
            .all(|character: Character| {
                 disp.push_str(format!("{}", character).as_str());
                 true
            });
    });
}

#[bench]
fn bench_screen_zip(b: &mut test::Bencher) {
    b.iter(|| {
        let mut disp: String = String::new();

        let screen: Display = Display::default();
        let pty_screen: PtyDisplay = PtyDisplay::default();

        pty_screen.into_iter()
           .zip(screen.into_iter())
           .all(|(pty_character, character)| {
                if character.is_space().not() {
                    disp.push_str(format!("{}", character).as_str());
                    true
                } else {
                    disp.push_str(format!("{}", pty_character).as_str());
                    true
                }
            });
    });
}
