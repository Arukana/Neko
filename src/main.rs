extern crate neko;

use neko::Neko;

fn main() {
    let mut neko: Neko = Neko::new(None, None).unwrap();

    while let Some(shell) = neko.next() {
        if let Some(()) = shell.is_output_screen() {
            print!("{}", neko)
        }
    }
}
