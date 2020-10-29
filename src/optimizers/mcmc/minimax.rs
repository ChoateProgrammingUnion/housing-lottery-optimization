use ballot::{Ballot, Student};
use optimizers::mcmc::{MCMCOptimizer, Proposal};
use optimizers::{Optimizer, generate_random_allocation};

#[derive(Clone)]
pub struct Minimax{
    ballots: Ballot
}

impl Minimax {
    #[allow(dead_code)]
    pub fn new(ballots: &Ballot) -> Self {
        Self {
            ballots: ballots.clone()
        }
    }

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

impl MCMCOptimizer for Minimax{

    // if current house is worse, chance of staying is current rank^(-2)
    // if current house is better, chance of moving is new rank^(-2)
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: Proposal) -> f64 {
        let extra_room_constant: usize = 1;
        let power_constant: f64 = -2.0;
        
        let student: &Student = &schedule[proposal.student_location.0][proposal.student_location.1];

        let mut current_house_rank = 1;
        let current_house_score = &student.ballot[proposal.student_location.0];

        let mut new_house_rank = 1;
        let new_house_score = &student.ballot[proposal.proposed_house];
        
        if schedule[proposal.proposed_house].len() >= self.ballots.houses[proposal.proposed_house].capacity+extra_room_constant {
            return 0.0;
        }

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
            let probability: f64 = 1.0 - (current_house_rank as f64 ).powf(power_constant);
            return probability;
        } else {
            let probability: f64 = (new_house_rank as f64).powf(power_constant);
            return probability;
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

        for house in schedule {
            counter += house.len();

            if counter > student_location {
                counter -= house.len();
                current_index = student_location - counter;
                break;
            }

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

impl Optimizer for Minimax {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut schedule: Vec<Vec<Student>> = generate_random_allocation(&self.ballots, 0 as u64);
        for _round in 0..rounds{
            schedule = self.step(schedule);
        }
        for house in 0..schedule.len(){
            while schedule[house].len()>self.ballots.houses[house].capacity {
                let student_location = self.gen_range(0, schedule[house].len());
                let student = schedule[house][student_location].clone();
                let choice = find_max(&self.ballots, &schedule, &student);
                schedule[house].remove(student_location);
                schedule[choice].push(student.clone());
            }
        }
        return schedule;
    }

    fn reseed(&mut self, _new_seed: u64) {
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}


fn find_max(ballots: &Ballot, schedule: &Vec<Vec<Student>>, student: &Student) -> usize {

    // finds highest ranked choice
    let mut max: Vec<f64> = vec![0.0,0.0];
    for i in 0..student.ballot.len() {
        if student.ballot[i] > max[0] {
            max[0] = student.ballot[i];
            max[1] = i as f64;
        }
    }

    // if there is space in the house, return that house
    if ballots.houses[max[1] as usize].capacity  > schedule[max[1] as usize].len(){
        return max[1] as usize;
    }
    // if there is no space in the highest ranked house, sets the highest ranking to 0.0, and tries again
    let mut new_ballot = student.clone();
    new_ballot.ballot[max[1] as usize] = 0.0;
    return find_max(ballots, schedule, &new_ballot);
}