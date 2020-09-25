use crate::ballot::Ballot;
use crate::ballot::Student;

pub trait Optimizer {
    fn new(ballots: &'static Ballot) -> Self; //

    fn ballots(&self) -> &'static Ballot;

    fn optimize(&self) -> Vec<Vec<Student>>;

    fn objective(&self) -> f64; // the objective function
}


/*
The null/identity optimizer. Just here for testing. Returns an empty schedule. Just for testing.
 */
impl Identity{
    
}

impl Identity for Optimizer{
    fn new(ballots: &'static Ballot) -> Identity{
        Identity {
            ballots: Ballot
        }
    }

    fn optimize(&self) -> Vec<Vec<Student>> {
        let mut schedule = vec![vec![Student], self.ballots.len(), self.ballots.len()];
        // for ballot in self.ballots.students.iter() {
        //     schedule.push(ballot.ballot)
        // }
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0;
    }
}

pub trait MCMCOptimizer: Optimizer {
    // An acceptance function takes in a particular location of the student (house, student) and the new house and returns a probability between 0-1 of acceptance.
    // 1 means a 100% probability of accepting
    fn acceptance(&self, (i32, i32), (i32)) -> f64;
    // A proposal function samples from all the house-student pairs and returns a students random change ((house, student), new_house).
    fn propose(&self) -> ((i32, i32), (i32));
}

impl MCMCNaive{
    
}

impl MCMCNaive for MCMCOptimizer{
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
