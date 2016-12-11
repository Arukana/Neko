extern crate neko;

use neko::Neko;
use std::io::Write;

fn main() {
    let mut neko: Neko = Neko::new(None, None).unwrap();

    while let Some(shell) = neko.next() {
        if let Some(()) = shell.is_output_screen() {
            print!("\x1B[H{}", format!("{}", neko));
        }
        if let Some(ref text) = shell.is_input_slice() {
            neko.write(text).unwrap();
            neko.flush().unwrap();
        }
    }
}
