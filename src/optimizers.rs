pub mod deans_algorithm;
pub mod deans_algorithm_friends;
pub mod identity;
pub mod mcmc;
pub mod mcmcswap;
pub mod multi_dist;
pub mod network;
pub mod random;
pub mod swap_naive;

use ballot::Ballot;
use ballot::Student;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub trait Optimizer {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>>;
    fn reseed(&mut self, new_seed: u64);
    fn objective(&self) -> f64; // the objective function to optimize
}

// Initial starting state generator
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

// Validates that the ballot is plausible and does not violate expectations
#[allow(dead_code)]
fn validate_ballot(ballot: &Ballot, schedule: Vec<Vec<Student>>) -> bool {
    let mut students = Vec::new();

    // Checks the number of houses
    assert_eq!(ballot.houses.len(), schedule.len());

    // Checks if each house is over capacity
    for (count, house) in schedule.iter().enumerate() {
        assert!(ballot.houses[count].capacity >= house.len());
        for student in house {
            students.push(student.clone());
        }
    }

    // Checks that students are not double-assigned
    for _ in 0..students.len() {
        let student = students.pop().expect("Empty datatype").clone();
        for other_student in &students {
            assert_ne!(student.name, other_student.name);
        }
    }

    true
}
