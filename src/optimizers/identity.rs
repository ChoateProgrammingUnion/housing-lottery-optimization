/*
The null/identity optimizer. Just here for testing. Returns an empty schedule. Just for testing.
 */
use ballot::{Ballot, Student};
use optimizers::{Optimizer, generate_random_allocation};

pub struct Identity {
    pub ballots: Ballot
}

impl Identity {
    #[allow(dead_code)]
    pub fn new(ballots: &Ballot) -> Identity {
        Identity {
            ballots: ballots.clone()
        }
    }
}

impl Optimizer for Identity {
    fn optimize(&mut self, _rounds: usize) -> Vec<Vec<Student>> {
        let schedule = generate_random_allocation(&self.ballots, 0);
        return schedule;
    }

    fn reseed(&mut self, _new_seed: u64) {

    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}
