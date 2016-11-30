extern crate neko;

use neko::Neko;
use std::io::Write;

fn main() {
    let mut neko: Neko = Neko::new(None, None).unwrap();

    while let Some(shell) = neko.next() {
        if let Some(()) = shell.is_output_screen() {
            neko.display_at((10, 10));
         // print!("\x1B[H{}", format!("{}", neko.get_screen())
            print!("{}", neko);
        }
        if let Some(ref text) = shell.is_input_slice() {
          neko.get_mut_shell().write(text).unwrap();
          neko.get_mut_shell().flush().unwrap();
        }
    }
}
