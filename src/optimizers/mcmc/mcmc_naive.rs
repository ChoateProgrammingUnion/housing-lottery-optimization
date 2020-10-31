use ballot::{Ballot, Student};
use optimizers::mcmc::{MCMCOptimizer, Proposal};
use optimizers::{Optimizer, generate_random_allocation};

#[derive(Clone)]
pub struct MCMCNaive{
    ballots: Ballot
}

impl MCMCNaive {
    #[allow(dead_code)]
    pub fn new(ballots: &Ballot) -> Self {
        Self {
            ballots: ballots.clone()
        }
    }

    // fn size(&self , schedule: Vec<Vec<Student>>) -> (Vec<Vec<Student>>, usize) {
    //     let mut counter = 0;
    //     for house in &schedule {
    //         counter += house.len();
    //     }
    //     return (schedule, counter);
    // }
    fn step(&self, mut schedule: Vec<Vec<Student>>) -> Vec<Vec<Student>> { // steps through one iteration of the MCMC chain
        let proposed_change: Proposal = self.propose(&schedule);
        let acceptance_prob: f64 = self.acceptance(&schedule,proposed_change.clone());
        // println!("{:?}", acceptance_prob);

        if self.gen_bool(acceptance_prob) { // proposal accepted
            let student = schedule[proposed_change.student_location.0].remove(proposed_change.student_location.1);
            schedule[proposed_change.proposed_house].push(student);
        }

        return schedule
    }
}

impl MCMCOptimizer for MCMCNaive{
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: Proposal) -> f64 {
        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];
        let ballot: Vec<f64> = student.clone().ballot;

        if schedule[proposal.proposed_house].len() >= self.ballots.houses[proposal.proposed_house].capacity {
            return 0 as f64;
        } else {
            return ballot[proposal.proposed_house];
        }
    }

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> Proposal {
        // Uniform, random sampling
        let size = self.ballots.students.len();

        let student_location = self.gen_range(0, size);
        let mut new_house = self.gen_range(0, schedule.len() -1);


        let mut counter: usize = 0;
        let mut current_house: usize = 0;
        let mut current_index: usize = 0;

        'house: for house in schedule {
            for _student in house {
                if counter == student_location {
                    break 'house;
                }
                counter += 1;
                current_index += 1;
            }
            current_index = 0;
            current_house += 1;
        }

        if new_house >= current_house { // ensure we don't get the same house
            new_house += 1;
        }

        let proposed_change = Proposal{
            student_location: (current_house, current_index),
            proposed_house: new_house
        };

        return proposed_change
    }
}

impl Optimizer for MCMCNaive {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = generate_random_allocation(&self.ballots, 0 as u64);
        for _round in 0..(rounds*1000){
            schedule = self.step(schedule);
        }
        return schedule;
    }

    fn reseed(&mut self, _new_seed: u64) {

    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}
