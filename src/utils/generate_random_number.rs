use rand::Rng;

pub fn generate(range: u32) -> u32 {
    rand::thread_rng().gen_range(1..range)
}
