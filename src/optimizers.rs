pub mod identity;
pub mod mcmc;

use ballot::Ballot;
use ballot::Student;

extern crate rand;

pub trait Optimizer {
    fn optimize(&self) -> Vec<Vec<Student>>;
    fn objective(&self) -> f64; // the objective function
}

pub(self) fn generate_random_allocation(ballot: &Ballot, seed: u64) -> Vec<Vec<Student>> {
    let mut schedule: Vec<Vec<Student>> = vec![vec![]; ballot.houses.len()];
    let mut rng: rand::SeedableRng = rand::SeedableRng::seed_from_u64(seed);
    for student in &ballot.students {
        let index = rng.
    }
    schedule
}