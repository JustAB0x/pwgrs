use rand::Rng;
use rand_chacha::ChaCha20Rng;

pub struct Charset {
    pub chars: Vec<char>,
}

impl Charset {
    pub fn new(chars: &[char]) -> Charset {
        Charset {
            chars: chars.to_vec(),
        }
    }

    pub fn get_rand_char(&self, rng: &mut ChaCha20Rng) -> char {
        let rand_index = rng.random_range(0..self.chars.len());
        self.chars[rand_index]
    }
}
