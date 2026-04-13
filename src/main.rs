use arboard::Clipboard;
use pwgrs::*;
use std::process;

fn main() {
    let options = pwgrs::Options::from_args();

    match run(&options) {
        Ok(passwords) => {
            if passwords.len() == 1 {
                if let Ok(mut clipboard) = Clipboard::new() {
                    let _ = clipboard.set_text(passwords[0].clone());
                }
            }

            for pass in passwords {
                println!("{}", pass);
            }
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
