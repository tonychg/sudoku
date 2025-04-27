use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

pub fn rng_from_seed(seed: u64) -> ChaCha8Rng {
    ChaCha8Rng::seed_from_u64(seed)
}

pub fn generate_seed() -> u64 {
    ChaCha8Rng::from_os_rng().random()
}
