use ballot::{Ballot, Student};
use optimizers::{Optimizer, generate_random_allocation};

use super::rand::rngs::StdRng;
use super::rand::distributions::WeightedIndex;
use super::rand::{SeedableRng, Rng};

pub struct MultiDist {
    pub ballots: Ballot,
    rng: StdRng
}

impl MultiDist {
    pub fn new(ballots: &Ballot, random_seed: u64) -> MultiDist {
        MultiDist {
            ballots: ballots.clone(),
            rng: StdRng::seed_from_u64(random_seed)
        }
    }

    fn do_random_move(&mut self, schedule: &Vec<Vec<Student>>) {
        let houses = &self.ballots.houses;
        let students = &self.ballots.students;

        /* Pick house to move student from */
        fn house_weight_for_student(student: &Student, house_id: usize) -> f64 {
            student.ballot_sum - student.ballot[house_id]
        }

        let mut house_weights = vec![0.0; houses.len()];
        for i in 0..houses.len() {
            for j in 0..schedule[i].len() {
                house_weights[i] += house_weight_for_student(&schedule[i][j], i);
            }
            println!("[move] [house] [gen_weights] House {} has a total weight of {}", i, house_weights[i]);
        }

        let mut house_dist = WeightedIndex::new(house_weights).unwrap();
        let house_index: usize = self.rng.sample(&house_dist);
        println!("[move] [house] ID {} ({}) was selected with probability {}/{}", house_index, houses[house_index].name, houses[house_index].capacity, students.len());
        // // Pick student in house
        // //   Sum up the weights and find the max weight TODO Define max weight
        // let mut total_weight: f64 = 0.0;
        // let mut max_weight: f64 = 0.0;
        // for student in &schedule[house_index] {
        //     let weight = student.ballot[house_index];
        //     total_weight += weight;
        //     if max_weight < weight {
        //         max_weight = weight;
        //     }
        // }
        // println!("[move] [house] House {} has a total weight of {} and a max weight of {}", house_index, total_weight, max_weight);
        // //  Define
        // self.rng.sample()
    }

    fn weight(&self, student: Student, house_id: usize) -> f64 {
        student.ballot[house_id]
    }
}

impl Optimizer for MultiDist {
    fn optimize(&mut self) -> Vec<Vec<Student>> {
        let mut schedule = generate_random_allocation(&self.ballots, 0);
        self.do_random_move(&schedule);
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}
