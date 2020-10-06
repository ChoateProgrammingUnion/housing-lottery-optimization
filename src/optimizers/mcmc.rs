pub mod mcmc_naive;

use optimizers::Optimizer;
use optimizers::rand::SeedableRng;
use optimizers::rand::Rng;
use super::rand::{thread_rng};
use ballot::Student;

#[derive(Debug, Clone)]
pub(self) struct Proposal {
    pub(self) student_location: (usize, usize),
    pub(self) proposed_house: usize
}

impl Proposal {
    fn new(student_location: (usize, usize), proposed_house: usize) -> Self {
        Self {
            student_location, proposed_house
        }
    }
}

pub(self) trait MCMCOptimizer: Optimizer {
    // An acceptance function takes in a particular location of the student (house, student) and the new house and returns a probability between 0-1 of acceptance.
    // 1 means a 100% probability of accepting
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: Proposal) -> f64;
    // A proposal function samples from all the house-student pairs and returns a students random change ((house, student), new_house).

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> Proposal;

    fn gen_bool(&self, prob: f64) -> bool {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: bool = rng.gen_bool(prob);
        return dice;
    }

    fn gen_range(&self, min: f64, max: f64) -> f64 {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: f64 = rng.gen_range(min, max);

        return dice;
    }

    fn step(&self, mut schedule: Vec<Vec<Student>>) -> Vec<Vec<Student>> { // steps through one iteration of the MCMC chain
        let proposed_change: Proposal = self.propose(&schedule);
        let acceptance_prob: f64 = self.acceptance(&schedule,proposed_change.clone());

        if self.gen_bool(acceptance_prob) { // proposal accepted
            let mut student = schedule[proposed_change.student_location.0].remove(proposed_change.student_location.1);
            schedule[proposed_change.proposed_house].push(student);
        }

        return schedule
    }
}

fn normalize(student: Student) -> Student {
    unimplemented!();
}
