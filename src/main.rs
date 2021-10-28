//! - Initial demand is generated based on a demand level (simulator input)
//! - At each EPOCH, generate Users on each region based on a poisson distribution multiplied by a weight (DEMAND)

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

fn generate_graph01() -> UnGraph<i32, ()> {
    UnGraph::<i32, ()>::from_edges(&[
        (0, 1), (0, 5),
        (1, 2), (1, 6),
        (2, 3), (2, 7), (2, 6),
        (3, 7), (3, 4), (3, 8),
        (4, 9), (4, 8),
        (5, 6), (5, 10),
        (6, 7), (6, 11), (6, 10),
        (7, 8), (7, 12), (7, 11),
        (8, 9), (8, 13), (8, 12),
        (9, 13),
        (10, 11), (10, 15),
        (11, 12), (11, 15),
        (12, 13), (12, 18), (12, 17), (12, 15),
        (13, 14), (13, 18),
        (14, 18),
        (15, 17), (15, 16),
        (16, 17),
        (17, 18),
        (18, 14)
    ])
}


fn main() {
    let mut rng = RandomGenerator::new(20.0);
    let graph = generate_graph01();
    println!("{:?}", Dot::with_config(&graph, &[Config::NodeIndexLabel, Config::EdgeNoLabel]));
}
