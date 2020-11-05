use ballot::{Ballot, Student};
use optimizers::{Optimizer, generate_random_allocation};

#[derive(Clone)]
pub struct RandomOptimizer {
    pub ballots: Ballot,
    seed: u64
}

impl RandomOptimizer {
    #[allow(dead_code)]
    pub fn new(ballots: &Ballot, random_seed: u64) -> RandomOptimizer {
        RandomOptimizer {
            ballots: ballots.clone(),
            seed: random_seed
        }
    }
}

impl Optimizer for RandomOptimizer {
    fn optimize(&mut self, _rounds: usize) -> Vec<Vec<Student>> {
        generate_random_allocation(&self.ballots, self.seed)
    }

    fn reseed(&mut self, new_seed: u64) {
        self.seed = new_seed
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}