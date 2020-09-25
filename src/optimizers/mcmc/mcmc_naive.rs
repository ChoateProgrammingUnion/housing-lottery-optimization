use ballot::{Ballot, Student};
use optimizers::mcmc::{MCMCOptimizer, Proposal};
use optimizers::Optimizer;

struct MCMCNaive{
    ballots: Ballot
}

impl MCMCNaive {
    fn new(ballots: Ballot) -> Self {
        Self {
            ballots
        }
    }
}

impl MCMCOptimizer for MCMCNaive{
    fn acceptance(&self, proposal: Proposal) -> f64 {
        unimplemented!()
    }

    fn propose(&self) -> Proposal {
        unimplemented!()
    }
}

impl Optimizer for MCMCNaive {
    fn optimize(&self) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}