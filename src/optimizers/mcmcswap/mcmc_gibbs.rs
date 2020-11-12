use ballot::{Ballot, Student};
use optimizers::mcmcswap::{MCMCOptimizerSWAP, ProposalSWAP};
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
    #[allow(dead_code)]
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
        
        //Person 1 current and proposed houses
        let current_house1 = &student.ballot[proposal.student_location.0];
        let proposed_house1 = &student.ballot[proposal.proposed_house.0];
        

        //Prioritizes friend
        let mut friend_weight_current = 0.0;
        let mut friend_weight_proposed = 0.0;
        let friends_list = &student.friends;
        let friend_constant = 1.0;

        
        for friend in friends_list{
            let friend_name = self.ballots.students[*friend].name.clone();
            for i in 0..schedule[proposal.student_location.0].len(){
                if schedule[proposal.student_location.0][i].name == friend_name{
                    friend_weight_current += friend_constant;

                }
            for i in 0..schedule[proposal.proposed_house.0].len(){
                if schedule[proposal.proposed_house.0][i].name == friend_name{
                    friend_weight_proposed += friend_constant;

                }
            }           
        }
        }
        
        
        //current number of students is less than overall capacity
        let current_total_weight = current_house1 * friend_weight_current;
        let proposed_total_weight = proposed_house1 * friend_weight_proposed;

        if schedule[proposal.proposed_house.0].len()<self.ballots.houses[proposal.proposed_house.0].capacity{
            if current_house1 < proposed_house1 && current_total_weight <= proposed_total_weight{
                    return 1 as f64;
                }else{
                    return 0 as f64;
                }
        }

        //Person 2 current and proposed houses
        let current_house2 = &student2.ballot[proposal.proposed_house.0];
        let proposed_house2 = &student2.ballot[proposal.student_location.0];

        //Person 2 friend weight calculations
        let mut friend_weight_current2 = 0.0;
        let mut friend_weight_proposed2 = 0.0;
        let friends_list2 = &student2.friends;
        
        for friend in friends_list2{
            let friend_name2 = self.ballots.students[*friend].name.clone();
            for i in 0..schedule[proposal.student_location.0].len(){
                if schedule[proposal.student_location.0][i].name == friend_name2{
                    friend_weight_current2 += friend_constant;

                }
            for i in 0..schedule[proposal.proposed_house.0].len(){
                if schedule[proposal.proposed_house.0][i].name == friend_name2{
                    friend_weight_proposed2 += friend_constant;
                    
                }
            }
        }
        }
        
        
        let current_total_weight2 = (current_house2) * friend_weight_current2;
        let proposed_total_weight2 = (proposed_house2) * friend_weight_proposed2;
        
        // swapping
        if current_house1<=proposed_house1 && current_house2 <= proposed_house2 &&(
        current_total_weight <= proposed_total_weight || current_total_weight2 <= proposed_total_weight2){
            return 1 as f64;
        } else {
            return 0 as f64;
        }

        /*
        if current_total_weight <= proposed_total_weight{
            if current_total_weight2 <= proposed_total_weight2{
                return ( proposed_house1 + proposed_house2)/(current_house1+current_house2) % 1 as f64
            } else{
                return 0 as f64;
            }*/ 
        
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
        for _round in 0..rounds{
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