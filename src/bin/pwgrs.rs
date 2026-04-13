use pwgrs::*;
use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand_core::SeedableRng;

fn main() {
    let mut rng = ChaCha20Rng::from_os_rng();
    let options = Options::from_args();

    match run(&options, &mut rng) {
        Ok(passwords) => {
            for pass in passwords {
                println!("{}", pass);
            }
        }
        Err(e) => {
            eprintln!("Application error: {}", e);
            std::process::exit(1);
        }
    }
}
