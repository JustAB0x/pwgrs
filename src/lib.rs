mod charset;
mod consts;
mod options;

pub use crate::charset::*;
pub use crate::consts::*;
pub use crate::options::*;

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub fn run(options: &Options) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut rng = ChaCha20Rng::from_os_rng();

    match options.subcommand {
        SubCommand::None(count) => {
            if count == 1 {
                Ok(vec![gen_password(&options.generator_options, &mut rng)])
            } else {
                let mut pass_list: Vec<String> = Vec::new();
                for _ in 0..count {
                    pass_list.push(gen_password(&options.generator_options, &mut rng));
                }
                Ok(pass_list)
            }
        }

        SubCommand::Secret(count) => {
            let charsets = vec![
                Charset::new(&CHARSET_ALPHABET),
                Charset::new(&CHARSET_ALPHABET_UPPERCASE),
                Charset::new(&CHARSET_NUMBERS),
            ];
            let options = GeneratorOptions {
                length: get_length_for_entropy(
                    MINIMUM_ENTROPY_IN_BITS,
                    count_chars_in_charsets(&charsets),
                ),
                charsets,
            };

            let mut pass_list: Vec<String> = Vec::new();
            for _ in 0..count {
                pass_list.push(gen_password(&options, &mut rng));
            }
            Ok(pass_list)
        }

        SubCommand::WiFi(count) => {
            let charsets = vec![
                Charset::new(&CHARSET_ALPHABET),
                Charset::new(&CHARSET_NUMBERS),
            ];
            let options = GeneratorOptions {
                length: get_length_for_entropy(
                    WIFI_ENTROPY_IN_BITS,
                    count_chars_in_charsets(&charsets),
                ),
                charsets,
            };

            let mut pass_list: Vec<String> = Vec::new();
            for _ in 0..count {
                let pass = gen_password(&options, &mut rng);
                pass_list.push(format!(
                    "{}-{}-{}-{}",
                    &pass[0..4],
                    &pass[4..8],
                    &pass[8..12],
                    &pass[12..16]
                ));
            }
            Ok(pass_list)
        }
    }
}

pub fn gen_password(options: &GeneratorOptions, rng: &mut ChaCha20Rng) -> String {
    let mut password = String::new();

    for _ in 0..options.length {
        let charset = get_rand_charset(&options.charsets, rng);
        password.push(charset.get_rand_char(rng));
    }

    password
}

fn get_rand_charset<'a>(charsets: &'a Vec<Charset>, rng: &mut ChaCha20Rng) -> &'a Charset {
    let rand_index = rng.random_range(0..charsets.len());
    &charsets[rand_index]
}

pub fn count_chars_in_charsets(charsets: &Vec<Charset>) -> u32 {
    let mut count: u32 = 0;
    for charset in charsets {
        count += charset.chars.len() as u32;
    }
    count
}

pub fn get_length_for_entropy(bits: u32, distinct_chars: u32) -> u32 {
    if distinct_chars == 0 || distinct_chars == 1 {
        return bits;
    }
    let log2_distinct = (distinct_chars as f64).ln() / 2_f64.ln();
    (bits as f64 / log2_distinct).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_256_bits_with_62_chars() {
        let length = get_length_for_entropy(256, 62);
        assert_eq!(length, 43);
    }

    #[test]
    fn test_gen_password_returns_correct_length() {
        let charsets = vec![Charset::new(&CHARSET_ALPHABET)];
        let options = GeneratorOptions {
            length: 10,
            charsets,
        };
        let mut rng = ChaCha20Rng::from_seed([0u8; 32]);
        let password = gen_password(&options, &mut rng);
        assert_eq!(password.len(), 10);
    }

    #[test]
    fn test_entropy_calculation_edge_cases() {
        assert_eq!(get_length_for_entropy(256, 2), 256);
        assert_eq!(get_length_for_entropy(128, 10), 39);
    }

    #[test]
    fn test_subcommand_count() {
        use crate::options::Options;
        use crate::options::SubCommand;

        let options = Options::from_args();
        match options.subcommand {
            SubCommand::None(c) => assert!(c >= 1),
            SubCommand::Secret(c) => assert!(c >= 1),
            SubCommand::WiFi(c) => assert!(c >= 1),
        }
    }

    #[test]
    fn test_count_chars_in_charsets() {
        let charsets = vec![
            Charset::new(&CHARSET_ALPHABET),
            Charset::new(&CHARSET_NUMBERS),
        ];
        assert_eq!(count_chars_in_charsets(&charsets), 36);
    }
}
