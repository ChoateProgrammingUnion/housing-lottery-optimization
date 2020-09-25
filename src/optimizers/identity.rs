/*
The null/identity optimizer. Just here for testing. Returns an empty schedule. Just for testing.
 */
use ballot::{Ballot, Student};
use optimizers::Optimizer;

pub struct Identity {
    pub ballots: Ballot
}

impl Identity {
    pub fn new(ballots: Ballot) -> Identity {
        Identity {
            ballots
        }
    }
}

impl Optimizer for Identity {
    fn optimize(&self) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}