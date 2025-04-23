use rand::Rng;
use rand_chacha::*;
use rand_core::SeedableRng;

pub struct Charset {
  pub chars: Vec<char>,
}

impl Charset {
  pub fn new(chars: &[char]) -> Charset {
    Charset {
      chars: chars.to_vec(),
    }
  }

  pub fn get_rand_char(&self) -> char {
    let mut rng = ChaCha20Rng::from_os_rng();
    let rand_index = rng.random_range(0..self.chars.len());
    self.chars[rand_index]
  }
}
