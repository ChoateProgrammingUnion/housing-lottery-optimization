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
}

impl MCMCOptimizerSWAP for MCMCSWAP{
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: ProposalSWAP) -> f64 {
        // variables with 1 or nothing refers to the first student
        // variable with 2 refers to the second student 

        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];

        let current_house1 = &student.ballot[proposal.student_location.0];
        let proposed_house1 = &student.ballot[proposal.proposed_house.0];

        let friends = &student.friends;
        let mut friend_weight_current = 0.0;
        let mut friend_weight_proposed = 0.0;

        
        for friend in friends{
            let name = self.ballots.students[*friend].name.clone();
            // find friends in current house
            'current1: for i in 0..schedule[proposal.student_location.0].len(){
                if schedule[proposal.student_location.0][i].name == name{
                    friend_weight_current += 0.1;
                    //break 'current1
                }
            }
            // find friends in proposed house
            'proposed1: for i in 0..schedule[proposal.proposed_house.0].len(){
                if schedule[proposal.proposed_house.0][i].name == name{
                    friend_weight_proposed += 0.1;
                    //break 'proposed1
                }
            }
        }
        

        let total_weight_current = current_house1 * (1.0 + friend_weight_current);
        let total_weight_proposed = proposed_house1 * (1.0 + friend_weight_proposed);
        
        // if proposal function found a empty house (only if there's more capacity than people)
        if proposal.proposed_house.1 == 1000{
            // metropolis hasting algorithm
            if total_weight_current <= total_weight_proposed{
                return 1 as f64;
            }else{
                return 0 as f64;
                // return total_weight_proposed / total_weight_current as f64;
            }
        }

        let student2: &Student = &schedule[proposal.proposed_house.0][proposal.proposed_house.1];

        let current_house2 = &student2.ballot[proposal.proposed_house.0];
        let proposed_house2 = &student2.ballot[proposal.student_location.0];

        let friends = &student2.friends;
        let mut friend_weight_current2 = 0.0;
        let mut friend_weight_proposed2 = 0.0;
        
        // same like the friend checking for first student
        for friend in friends{
            let name2 = self.ballots.students[*friend].name.clone();
            'proposed2: for i in 0..schedule[proposal.student_location.0].len(){
                if schedule[proposal.student_location.0][i].name == name2{
                    friend_weight_proposed2 += 0.1;
                    //break 'proposed2
                }
            }
            'current2: for i in 0..schedule[proposal.proposed_house.0].len(){
                if schedule[proposal.proposed_house.0][i].name == name2{
                    friend_weight_current2 += 0.1;
                    //break 'current2
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
            // return (total_weight_proposed + total_weight_proposed2) / (total_weight_current + total_weight_current2) as f64
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
            for _student in 0..schedule[house].len(){
                if counter as f64 == student_number as f64{
                    break 'house
                }
                current_index += 1;
                counter += 1;
            }
        }
        


        let mut current_house2 = self.gen_range(0,schedule.len());
        while current_house2 == current_house{
            current_house2 = self.gen_range(0,schedule.len());
        }


        // if house is not full, no swap (this only happens if there's more capacity than people)
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
        for _round in 0..rounds{
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