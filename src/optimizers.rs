pub mod identity;
pub mod mcmc;
pub mod multi_dist;

use ballot::Ballot;
use ballot::Student;

extern crate rand;
use self::rand::rngs::StdRng;
use self::rand::{SeedableRng, Rng};

pub trait Optimizer {
    fn optimize(&mut self) -> Vec<Vec<Student>>;
    fn objective(&self) -> f64; // the objective function
}

pub fn generate_random_allocation(ballot: &Ballot, seed: u64) -> Vec<Vec<Student>> {
    let mut schedule: Vec<Vec<Student>> = vec![vec![]; ballot.houses.len()];
    let mut rng = StdRng::seed_from_u64(seed);
    for student in &ballot.students {
        loop {
            let index = rng.gen_range(0, schedule.len());
            if schedule[index].len() < ballot.houses[index].capacity {
                schedule[index].push(student.clone());
                break;
            }
        }
    }
    schedule
}