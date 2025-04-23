mod config;
mod generator;

use crate::config::Config;
use crate::generator::{calculate_entropy, generate_password};
use clap::Parser;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    let mut rng = ChaCha20Rng::from_entropy();
    let password = generate_password(&config, &mut rng)
        .map_err(|e| format!("Error generating password: {}", e))?;

    println!("Generated password: {}", password);

    let charset_size = [!config.no_lowercase as usize * generator::LOWERCASE.len(),
                        !config.no_uppercase as usize * generator::UPPERCASE.len(),
                        !config.no_numbers as usize * generator::NUMBERS.len(),
                        !config.no_symbols as usize * generator::SYMBOLS.len()]
        .iter().sum::<usize>();
    let entropy = calculate_entropy(config.length, charset_size);
    println!("Password entropy: {:.2} bits", entropy);

    Ok(())
}