pub mod mcmc_naive;
pub mod minimax;
pub mod mcmc_swap;

use optimizers::Optimizer;
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

    fn gen_range(&self, min: usize, max: usize) -> usize {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: usize = rng.gen_range(min, max);

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

#[derive(Debug, Clone)]
pub(self) struct ProposalSWAP {
    pub(self) student_location: (usize, usize),
    pub(self) proposed_house: (usize,usize)
}

impl ProposalSWAP {
    fn new(student_location: (usize, usize), proposed_house: (usize,usize)) -> Self {
        Self {
            student_location, proposed_house
        }
    }
}

pub(self) trait MCMCOptimizerSWAP: Optimizer {
    // An acceptance function takes in a particular location of the student (house, student) and the new house and returns a probability between 0-1 of acceptance.
    // 1 means a 100% probability of accepting
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: ProposalSWAP) -> f64;
    // A proposal function samples from all the house-student pairs and returns a students random change ((house, student), new_house).

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> ProposalSWAP;

    fn gen_bool(&self, prob: f64) -> bool {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: bool = rng.gen_bool(prob);
        return dice;
    }

    fn gen_range(&self, min: usize, max: usize) -> usize {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: usize = rng.gen_range(min, max);

        return dice;
    }

    fn step(&self, mut schedule: Vec<Vec<Student>>) -> Vec<Vec<Student>> { // steps through one iteration of the MCMC chain
        let proposed_change: ProposalSWAP = self.propose(&schedule);
        let acceptance_prob: f64 = self.acceptance(&schedule,proposed_change.clone());

        if self.gen_bool(acceptance_prob) { // proposal accepted
            let mut student = schedule[proposed_change.student_location.0].remove(proposed_change.student_location.1);
            let mut student2 = schedule[proposed_change.proposed_house.0].remove(proposed_change.proposed_house.1);
            schedule[proposed_change.proposed_house.0].push(student);
            schedule[proposed_change.student_location.0].push(student2);
        }

        return schedule
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    use ballot::Ballot;

    fn validate_ballot(ballot: &Ballot, schedule: Vec<Vec<ballot::Student>>) -> bool{
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

    #[test]
    fn test_mcmc_swap() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut naive = optimizers::mcmc::mcmc_swap::MCMCSWAP::new(&input_ballot);

        assert!(validate_ballot(&input_ballot, naive.optimize(0)));
        assert!(validate_ballot(&input_ballot, naive.optimize(1)));
        assert!(validate_ballot(&input_ballot, naive.optimize(100)));
    }

    #[test]
    fn test_mcmc_naive() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut naive = optimizers::mcmc::mcmc_naive::MCMCNaive::new(&input_ballot);

        assert!(validate_ballot(&input_ballot, naive.optimize(0)));
        assert!(validate_ballot(&input_ballot, naive.optimize(1)));
        assert!(validate_ballot(&input_ballot, naive.optimize(100)));
    }

    #[test]
    fn test_deans_algo(){
        let input_ballot = input::load_input(ballot::normalize);

        let mut dean = optimizers::deans_algorithm::DeansAlgorithm::new(&input_ballot);

        assert!(validate_ballot(&input_ballot, dean.optimize(0)));
        assert!(validate_ballot(&input_ballot, dean.optimize(1)));
        assert!(validate_ballot(&input_ballot, dean.optimize(100)));
    }

    #[test]
    fn test_multi_dist(){
        let input_ballot = input::load_input(ballot::normalize);

        let mut multi = optimizers::multi_dist::MultiDist::new(&input_ballot, 0, 10.0);

        assert!(validate_ballot(&input_ballot, multi.optimize(0)));
        assert!(validate_ballot(&input_ballot, multi.optimize(1)));
        assert!(validate_ballot(&input_ballot, multi.optimize(100)));
    }
}