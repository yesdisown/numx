mod addition;

pub trait Exercise {
    fn generate(rng: rand::rngs::ThreadRng, min: isize, max: isize) -> Self;
}
