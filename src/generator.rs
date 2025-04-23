use rand::Rng;
use rand_chacha::ChaCha20Rng;

pub const LOWERCASE: &[u8] = b"abcdefghjkmnpqrstuvwxyz";
pub const UPPERCASE: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ";
pub const NUMBERS: &[u8] = b"23456789";
pub const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn calculate_entropy(length: u32, charset_size: usize) -> f64 {
    (length as f64) * (charset_size as f64).log2()
}

pub fn generate_password(config: &super::config::Config, rng: &mut ChaCha20Rng) -> Result<String, String> {
    if config.no_lowercase && config.no_uppercase && config.no_numbers && config.no_symbols {
        return Err("At least one character type must be enabled".to_string());
    }
    if config.length == 0 {
        return Err("Password length must be greater than 0".to_string());
    }
    if config.length > 10000 {
        return Err("Password length must not exceed 10,000".to_string());
    }

    let mut charsets = Vec::new();
    if !config.no_lowercase {
        charsets.push(LOWERCASE);
    }
    if !config.no_uppercase {
        charsets.push(UPPERCASE);
    }
    if !config.no_numbers {
        charsets.push(NUMBERS);
    }
    if !config.no_symbols {
        charsets.push(SYMBOLS);
    }

    let charset_size = charsets.iter().map(|cs| cs.len()).sum::<usize>();
    let mut password = String::with_capacity(config.length as usize);

    for _ in 0..config.length {
        let mut idx = rng.gen_range(0..charset_size);
        for &charset in &charsets {
            if idx < charset.len() {
                password.push(charset[idx] as char);
                break;
            }
            idx -= charset.len();
        }
    }

    Ok(password)
}