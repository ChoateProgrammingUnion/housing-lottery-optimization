use ballot::{Ballot, Student};
use optimizers::mcmcswap::{MCMCOptimizerSWAP, ProposalSWAP};
use optimizers::{Optimizer, generate_random_allocation};

#[derive(Clone)]
pub struct MCMCSWAP{
    ballots: Ballot
}

impl MCMCSWAP {
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

impl MCMCOptimizerSWAP for MCMCSWAP{
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: ProposalSWAP) -> f64 {
        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];
        let current_house1 = &student.ballot[proposal.student_location.0];
        let proposed_house1 = &student.ballot[proposal.proposed_house.0];
        let mut friend_weight_current = 0.0;
        let mut friend_weight_proposed = 0.0;

        let friends = &student.friends;

        for friend in friends{
            for i in 0..schedule[proposal.student_location.0].len(){
                if schedule[proposal.student_location.0][i].name == self.ballots.students[*friend].name{
                    friend_weight_current += 1.0;
                }
            }
            for i in 0..schedule[proposal.proposed_house.0].len(){
                if schedule[proposal.proposed_house.0][i].name == self.ballots.students[*friend].name{
                    friend_weight_proposed += 1.0;
                }
            }
        }

        let total_weight_current = current_house1 * (1.0 + friend_weight_current);
        let total_weight_proposed = proposed_house1 * (1.0 + friend_weight_proposed);
        
        // if proposal function found a empty house (only if there's more capacity than people)
        if proposal.proposed_house.1 == 1000{
            println!("happened");
            if total_weight_current <= total_weight_proposed{
                return 1 as f64;
            }else{
                return 0 as f64;
            }
        }

        let student2: &Student = &schedule[proposal.proposed_house.0][proposal.proposed_house.1];

        let current_house2 = &student2.ballot[proposal.proposed_house.0];
        let proposed_house2 = &student2.ballot[proposal.student_location.0];

        let friends = &student2.friends;
        let mut friend_weight_current2 = 0.0;
        let mut friend_weight_proposed2 = 0.0;

        for friend in friends{
            for i in 0..schedule[proposal.student_location.0].len(){
                if schedule[proposal.student_location.0][i].name == self.ballots.students[*friend].name{
                    friend_weight_proposed2 += 1.0;
                }
            }
            for i in 0..schedule[proposal.proposed_house.0].len(){
                if schedule[proposal.proposed_house.0][i].name == self.ballots.students[*friend].name{
                    friend_weight_current2 += 1.0;
                }
            }
        }
        
        let total_weight_current2 = current_house2 * (1.0 + friend_weight_current2);
        let total_weight_proposed2 = proposed_house2 * (1.0 + friend_weight_proposed2);

        // metropolis hasting algorithm
        if total_weight_current + total_weight_current2 <= total_weight_proposed + total_weight_proposed2 {
            return 1 as f64;
        } else {
            return 0 as f64
        }
    }

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> ProposalSWAP {
        // same as other mcmc code
        let size = self.ballots.students.len();
        let student_number = self.gen_range(0, size);
        let mut current_house = 0;
        let mut current_index = 0;
        let mut counter = 0;
        'house: for house in 0..schedule.len(){
            current_index = 0;
            current_house = house;
            for student in 0..schedule[house].len(){
                if counter as f64 == student_number as f64{
                    break 'house
                }
                current_index += 1;
                counter += 1;
            }
        }
        
        //let current_house = self.gen_range(0,schedule.len());

        let mut current_house2 = self.gen_range(0,schedule.len());
        while current_house2 == current_house{
            current_house2 = self.gen_range(0,schedule.len());
        }
        //let current_index = self.gen_range(0,schedule[current_house].len());

        // if house is not full, no swap (only if there's more capacity than people)
        if self.ballots.houses[current_house2].capacity > schedule[current_house2].len(){
            let proposed_change = ProposalSWAP{
                student_location: (current_house, current_index),
                proposed_house: (current_house2, 1000)
            };
            return proposed_change
        }

        let current_index2 = self.gen_range(0,schedule[current_house2].len());

        let proposed_change = ProposalSWAP{
            student_location: (current_house, current_index),
            proposed_house: (current_house2, current_index2)
        };

        return proposed_change
    }
}


impl Optimizer for MCMCSWAP {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = generate_random_allocation(&self.ballots, 0 as u64);
        for round in 0..rounds{
            schedule = self.step(schedule);
        }
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }

    fn reseed(&mut self, _new_seed: u64) {

    }
}