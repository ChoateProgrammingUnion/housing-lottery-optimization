use crate::ballot::Ballot;
use crate::ballot::Student;

trait Optimizer {
    fn new(ballots: &'static Ballot) -> Self; //

    fn ballots(&self) -> &'static Ballot;

    fn optimize(&self) -> Vec<Vec<Student>>;
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
        let schedule = vec![vec![Student], self.ballots.len(), self.ballots.len()]
        return schedule
    }
}
