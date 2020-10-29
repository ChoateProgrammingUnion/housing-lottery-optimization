pub mod identity;
pub mod mcmc;
pub mod mcmcswap;
pub mod deans_algorithm;
pub mod multi_dist;
pub mod network;


use ballot::Ballot;
use ballot::Student;

use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};

pub trait Optimizer {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>>;
    fn reseed(&mut self, new_seed: u64);
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

fn validate_ballot(ballot: &Ballot, schedule: Vec<Vec<Student>>) -> bool{
    let students_total = ballot.students.len();
    let mut students = Vec::new();

    assert_eq!(ballot.houses.len(), schedule.len());

    for (count, house) in schedule.iter().enumerate() {
        assert!(ballot.houses[count].capacity >= house.len());
        for student in house {
            students.push(student.clone());
        }
    }

    for student in 0..students.len() {
        let mut student = students.pop().expect("Empty datatype").clone();
        for other_student in &students {
            assert_ne!(student.name, other_student.name);
        }
    }

    true
}
