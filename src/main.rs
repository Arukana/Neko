#[macro_use]
extern crate clap;
extern crate neko;

use neko::Neko;
use std::io::Write;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    let mut neko: Neko = Neko::new(
        options.value_of("repeat").and_then(|repeat| repeat.parse::<i64>().ok()),
        options.value_of("interval").and_then(|interval| interval.parse::<i64>().ok()),
        options.value_of("command"),
        None
    ).unwrap();

    while let Some(shell) = neko.next() {
        if let Some(()) = shell.is_output_screen() {
           // print!("\x1B[H{}", format!("{}", neko));
            print!("{}", format!("{}", neko));
        }
        if let Some(ref text) = shell.is_input_slice() {
            neko.write(text).unwrap();
            neko.flush().unwrap();
        }
    }
}
