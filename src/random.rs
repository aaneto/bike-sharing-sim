use rand::{
    RngCore,
    rngs::OsRng,
    distributions::Distribution
};
use statrs::distribution::Uniform;

pub fn uniform_0_to_1() -> f64 {
    let mut rng = OsRng {};

    let u = Uniform::new(0.0, 1.0).expect("Cannot generate uniform number");

    u.sample(&mut rng)
}

pub fn uniform_integer_0_end(end: u32) -> u32 {
    let mut rng = OsRng {};

    rng.next_u32() % end
}