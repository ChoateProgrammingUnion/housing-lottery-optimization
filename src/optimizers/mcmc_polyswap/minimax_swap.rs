use ballot::{Ballot, Student};
use optimizers::mcmc_polyswap::{MCMCOptimizer_polyswap, Proposal};
use optimizers::{Optimizer, generate_random_allocation};
#[derive(Clone)]

pub struct Minimax_swap{
    ballots: Ballot
}

impl Minimax_swap {
    pub fn new(ballots: &Ballot) -> Self {
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

impl MCMCOptimizer_polyswap for Minimax_swap{
    // if current house is worse, chance of staying is current rank^(-2)
    // if current house is better, chance of moving is new rank^(-2)
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: Proposal) -> f64 {
        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];

        let mut current_house_rank = 1;
        let current_house_score = &student.ballot[proposal.student_location.0];

        let mut new_house_rank = 1;
        let new_house_score = &student.ballot[proposal.proposed_house];


        // finds how many houses are higher scored than the house in question so the rank can be determined
        for house in &student.ballot{
            if house > current_house_score {
                current_house_rank+=1;
            }
            if house > new_house_score {
                new_house_rank+=1;
            }
        }

        // determines probability of changing houses based on the rank of each house
        if new_house_rank > current_house_rank {
            let probability: f64 = 1.0 - (current_house_rank as f64 ).powf(-2.0);
            return probability;
        } else {
            let probability: f64 = (new_house_rank as f64).powf(-2.0);
            return probability;
        }
    }

    fn propose(&self, schedule: &Vec<Vec<Student>>, required_houses: Vec<isize>) -> Proposal {
        let mut student_location = 0;
        let mut new_house = 0;


        let mut current_house: usize = 0;
        let mut current_index: usize = 0;
        if required_houses[0] == -1 {
            // Uniform, random sampling
            let size = self.ballots.students.len();

            student_location = self.gen_range(0, size);
            new_house = self.gen_range(0, schedule.len() -1);


            let mut counter: usize = 0;

            'house: for house in schedule {
                for student in house {
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
        } else {
            let size = schedule[required_houses[0] as usize].len();
            student_location = self.gen_range(0,size);
            current_house = required_houses[0] as usize;
            current_index = student_location;
            new_house = required_houses[1] as usize;
        }
        let proposed_change = Proposal{
            student_location: (current_house, current_index),
            proposed_house: new_house
        };

        return proposed_change
    }

    fn ballots(&self) -> &Ballot{
        return &self.ballots;
    }
}

impl Optimizer for Minimax_swap {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = generate_random_allocation(&self.ballots, 0 as u64);
        for round in 0..rounds{
            schedule = self.step(schedule, vec![-1,-1]);

        }
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
    fn reseed(&mut self, new_seed: u64) {}
}

#[cfg(test)]
mod tests {
    use crate::*;
    use ballot::Ballot;


    #[test]
    fn test_minimax_swap() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut minimax_swap = optimizers::mcmc_polyswap::minimax_swap::Minimax_swap::new(&input_ballot);

        assert!(optimizers::validate_ballot(&input_ballot, minimax_swap.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax_swap.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax_swap.optimize(100)));
    }

}

