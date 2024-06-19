use pwgrs::*;
use arboard::Clipboard;
use std::process;

fn main() {
  // Parse cli arguments
  let options = pwgrs::Options::from_args();

  match run(&options) {
    Ok(passwords) => {
      // If there is only one password being returned, copy it to clipboard
      // before printing to stdout.
      if passwords.len() == 1 {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(passwords[0].clone()).unwrap();
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
