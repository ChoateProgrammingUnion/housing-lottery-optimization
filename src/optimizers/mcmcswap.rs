pub mod mcmc_swap;

use optimizers::Optimizer;
use rand::{thread_rng, Rng};
use ballot::Student;

#[derive(Debug, Clone)]
pub(self) struct ProposalSWAP {
    pub(self) student_location: (usize, usize),
    pub(self) proposed_house: (usize,usize)
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
            schedule[proposed_change.proposed_house.0].push(student);
            
            // only swap second student if such student was selected
            if proposed_change.proposed_house.1 != 1000{
                let mut student2 = schedule[proposed_change.proposed_house.0].remove(proposed_change.proposed_house.1);
                schedule[proposed_change.student_location.0].push(student2);
            }
        }

        return schedule
    }
}