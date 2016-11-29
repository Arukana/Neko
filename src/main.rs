extern crate neko;

use neko::Neko;
use std::io::Write;

fn main() {
    let mut neko: Neko = Neko::new(None, None).unwrap();

    while let Some(shell) = neko.next() {
      if let Some(()) = shell.is_output_screen() {
        neko.display_at((0, 0));
        let the = neko.get_mut_shell();
        print!("\x1B[H{}", format!("{}", the)
       // print!("{}", format!("{}", the)
            .chars()
            .take(the.get_screen()
              .get_window_size()
              .row_by_col())
            .collect::<String>());
        }
        if let Some(ref text) = shell.is_input_slice() {
          neko.get_mut_shell().write(text).unwrap();
          neko.get_mut_shell().flush().unwrap();
        }
    }
}
