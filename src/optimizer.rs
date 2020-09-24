use crate::ballot::Ballot;
use crate::ballot::Student;

pub trait Optimizer {
    fn new(ballots: &'static Ballot) -> Self; //

    fn ballots(&self) -> &'static Ballot;

    fn optimize(&self) -> Vec<Vec<Student>>;

    fn objective(&self) -> f64; // the objective function
}

impl Identity{
    
}

impl Identity for Optimizer{
    fn new(ballots: &'static Ballot) -> Identity{
        Identity {
            ballots: Ballot
        }
    }

    fn optimize(&self) -> Vec<Vec<Student>> {
        let iter = self.ballots.iter();
        let schedule = vec![vec![Student], self.ballots.len(), self.ballots.len()];
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0;
    }
}

pub trait MCMCOptimizer: Optimizer {
    fn acceptance(&self, (i32, i32), (i32, i32)) -> f64;
    fn propose(&self) -> ((i32, i32), (i32, i32));
}

impl MCMC{
    
}

impl MCMC for MCMCOptimizer{
    fn new(ballots: &'static Ballot) -> Identity{
        Identity {
            ballots: Ballot
        }
    }

    fn optimize(&self) -> Vec<Vec<Student>> {
        let iter = self.ballots.iter();
        let schedule = vec![vec![Student], self.ballots.len(), self.ballots.len()];
        return schedule;
    }
}
