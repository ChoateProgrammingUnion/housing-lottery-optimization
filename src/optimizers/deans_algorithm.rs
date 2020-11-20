/*
implementation of the algorithm the deans use
*/

use ballot::{Ballot, Student};
use optimizers::Optimizer;
use rand::Rng;

#[derive(Clone)]
pub struct DeansAlgorithm {
    pub ballots: Ballot,
}

impl DeansAlgorithm {
    #[allow(dead_code)]
    pub fn new(ballots: &Ballot) -> Self {
        Self {
            ballots: ballots.clone(),
        }
    }

    // returns the index of the highest ranked availible choice for a given student
    fn find_max(&self, ballots: &Ballot, schedule: &Vec<Vec<Student>>, student: &Student) -> usize {
        // finds highest ranked choice
        let mut max: Vec<f64> = vec![0.0, 0.0];
        for i in 0..student.ballot.len() {
            if student.ballot[i] > max[0] {
                max[0] = student.ballot[i];
                max[1] = i as f64;
            }
        }

        // if there is space in the house, return that house
        if ballots.houses[max[1] as usize].capacity > schedule[max[1] as usize].len() {
            return max[1] as usize;
        }
        // if there is no space in the highest ranked house, sets the highest ranking to 0.0, and tries again
        let mut new_ballot = student.clone();
        new_ballot.ballot[max[1] as usize] = 0.0;
        return self.find_max(ballots, schedule, &new_ballot);
    }
}

impl Optimizer for DeansAlgorithm {
    fn optimize(&mut self, _rounds: usize) -> Vec<Vec<Student>> {
        // create a vector that includes a vector for each house
        let mut schedule: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];

        let mut input = self.ballots.students.clone();

        // chooses a student at random, finds their most prefered available house, and places that student into the respective house
        for _ in 0..input.len() {
            let mut rng = rand::thread_rng();
            let len: f64 = input.len() as f64;
            let rand_num: f64 = rng.gen();

            //random index for the student
            let index_choice: usize = (len*rand_num) as usize;

            // student that was chosen
            let choice = input[index_choice].clone();

            // index of most prefered house
            let preference = self.find_max(&self.ballots, &schedule, &choice);

            // removes the student and adds them to the house
            input.remove(index_choice);
            schedule[preference].push(choice.clone());
        }
        schedule
    }

    fn reseed(&mut self, _new_seed: u64) {}

    // doesn't use objective function, so it just returns 0.0
    fn objective(&self) -> f64 {
        0.0
    }
}
