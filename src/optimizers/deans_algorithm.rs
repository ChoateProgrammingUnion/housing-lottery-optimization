/*
implementation of the algorithm the deans use
*/

use ballot::{Ballot, Student};
use optimizers::Optimizer;
use optimizers::rand::SeedableRng;
use optimizers::rand::Rng;
use optimizers::rand::seq::IteratorRandom;

pub struct DeansAlgorithm {
    pub ballots: Ballot
}

impl DeansAlgorithm{
    pub fn new(ballots: Ballot) -> Self {
        Self {
            ballots
        }
    }
    fn size(&self , schedule: Vec<Vec<Student>>) -> (Vec<Vec<Student>>, usize) {
        let mut counter = 0;
        for house in &schedule {
            counter += house.len();
        }
        return (schedule, counter);
    }


    // Find most wanted house of student
    fn find_max(&self, ballots: &Ballot, schedule: &Vec<Vec<Student>>, student: &Student) -> usize {
        let mut max: Vec<f64> = vec![0.0,0.0];
        for i in 0..student.ballot.len() {
            if student.ballot[i] > max[0] {
                max[0] = student.ballot[i];
                max[1] = i as f64;
            }
        }
        if ballots.houses[max[1] as usize].capacity  > schedule[max[1] as usize].len(){
            return max[1] as usize;
        }
        let mut new_ballot = student.clone();
        new_ballot.ballot[max[1] as usize] = 0.0;
        return self.find_max(ballots, schedule, &new_ballot);
    }
}

impl Optimizer for DeansAlgorithm{
    fn optimize(&self) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];
        let mut input = &self.ballots.students;

        // go through each student at random and do stuff
        for i in 0..input.len() {
            let mut rng = rand::thread_rng();
            let choice = input.iter().choose(&mut rng).unwrap();
            let preference = self.find_max(&self.ballots, &schedule, choice);
            schedule[preference].push(choice.clone());

        }
        schedule
    }

   
    fn objective(&self) -> f64 {
        0.0
    }
}
