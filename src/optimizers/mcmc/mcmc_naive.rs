use ballot::{Ballot, Student};
use optimizers::mcmc::{MCMCOptimizer, Proposal};
use optimizers::Optimizer;

struct MCMCNaive{
    ballots: Ballot
}

impl MCMCNaive {
    fn new(ballots: &Ballot) -> Self {
        Self {
            ballots: ballots.clone()
        }
    }
    fn size(&self , schedule: Vec<Vec<Student>>) -> (Vec<Vec<Student>>, usize) {
        let mut counter = 0;
        for house in &schedule {
            counter += house.len();
        }
        return (schedule, counter);
    }
}

impl MCMCOptimizer for MCMCNaive{
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: Proposal) -> f64 {
        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];
        let ballot: Vec<f64> = student.clone().ballot;

        return ballot[proposal.proposed_house];
    }

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> Proposal {
        // Uniform, random sampling
        let size = self.ballots.students.len();

        let student = self.gen_range(0 as f64, size as f64) as usize;
        let mut house = self.gen_range(0 as f64, (schedule.len() -1) as f64) as usize;

        if house >= schedule.len() { // ensure we don't get the same house
            house += 1;
        }

        return Proposal{
            student_location: ((student as f64 / schedule.len() as f64).floor() as usize, student % schedule.len()),
            proposed_house: house
        }
    }
}

impl Optimizer for MCMCNaive {
    fn optimize(&mut self) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}
