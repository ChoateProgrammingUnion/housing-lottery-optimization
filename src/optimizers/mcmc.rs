pub mod mcmc_naive;

use optimizers::Optimizer;

pub(self) struct Proposal {
    pub(self) student_location: (i32, i32),
    pub(self) proposed_house: i32
}

impl Proposal {
    fn new(student_location: (i32, i32), proposed_house: i32) -> Self {
        Self {
            student_location, proposed_house
        }
    }
}

pub(self) trait MCMCOptimizer: Optimizer {
    // An acceptance function takes in a particular location of the student (house, student) and the new house and returns a probability between 0-1 of acceptance.
    // 1 means a 100% probability of accepting
    fn acceptance(&self, proposal: Proposal) -> f64;
    // A proposal function samples from all the house-student pairs and returns a students random change ((house, student), new_house).
    fn propose(&self) -> Proposal;
}
