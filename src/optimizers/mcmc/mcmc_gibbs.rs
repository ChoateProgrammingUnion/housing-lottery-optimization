use ballot::{Ballot, Student};
use optimizers::mcmc::{MCMCOptimizerSWAP, ProposalSWAP};
use optimizers::{Optimizer, generate_random_allocation};

#[derive(Clone)]
pub struct MCMCGibbs{
    ballots: Ballot
}

impl MCMCGibbs {
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

impl MCMCOptimizerSWAP for MCMCGibbs{
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: ProposalSWAP) -> f64 {
        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];
        let student2: &Student = &schedule[proposal.proposed_house.0][proposal.proposed_house.1];

        let current_house1 = &student.ballot[proposal.student_location.0];
        let proposed_house1 = &student.ballot[proposal.proposed_house.0];

        //current number of students is less than overall capacity
        if schedule[proposal.proposed_house.0].len()<self.ballots.houses[proposal.proposed_house.0].capacity{
            if current_house1 <= proposed_house1{
                    return (current_house1/proposed_house1) as f64;
                }else{
                    return 0 as f64;
                }
        }

        let current_house2 = &student2.ballot[proposal.proposed_house.0];
        let proposed_house2 = &student2.ballot[proposal.student_location.0];
        
        if current_house1 + current_house2 <= proposed_house1 + proposed_house2 {
            return (proposed_house1 + proposed_house2)/(current_house1 + current_house2) % 1 as f64;
        } else {
            return 0 as f64;
        }
    }

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> ProposalSWAP {
        let current_house = self.gen_range(0,schedule.len());
        let mut current_house2 = self.gen_range(0,schedule.len());
        while current_house2 == current_house{
            current_house2 = self.gen_range(0,schedule.len());
        }
        let current_index = self.gen_range(0,schedule[current_house].len());
        let current_index2 = self.gen_range(0,schedule[current_house2].len());

        let proposed_change = ProposalSWAP{
            student_location: (current_house, current_index),
            proposed_house: (current_house2, current_index2)
        };

        return proposed_change
    }
}


impl Optimizer for MCMCGibbs {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = generate_random_allocation(&self.ballots, 0 as u64);
        for round in 0..rounds{
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