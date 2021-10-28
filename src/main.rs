/**
 * - Initial demand is generated based on a demand level (simulator input)
 *
 *
 *
 *
 *
*/


use petgraph::dot::{Config, Dot};
use petgraph::graph::{UnGraph};
use statrs::distribution::Poisson;
use rand::{rngs::OsRng, distributions::Distribution};

struct RandomGenerator {
    poisson: Poisson,
    rng: OsRng
}

impl RandomGenerator {
    pub fn new(p_lambda: f64) -> RandomGenerator {
        let rng = OsRng {};
        let poisson = Poisson::new(p_lambda).expect("Cannot generate poisson distribution.");

        RandomGenerator {
            rng, poisson
        }
    }

    pub fn gen_poisson(&mut self) -> f64 {
        self.poisson.sample(&mut self.rng)
    }
}


fn main() {
    let mut rng = RandomGenerator::new(20.0);
    dbg!(rng.gen_poisson());

    let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);
    println!("{:?}", Dot::with_config(&g, &[Config::NodeIndexLabel, Config::EdgeNoLabel]));
}
