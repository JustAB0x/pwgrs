use rand::Rng;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;

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
    let mut rng = ChaCha20Rng::from_entropy();
    let rand_index = rng.gen_range(0..self.chars.len());
    self.chars[rand_index]
  }
}
