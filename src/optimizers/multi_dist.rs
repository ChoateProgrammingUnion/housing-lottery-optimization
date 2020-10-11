use ballot::{Ballot, Student, House};
use optimizers::{Optimizer, generate_random_allocation};

use super::rand::rngs::StdRng;
use super::rand::distributions::WeightedIndex;
use super::rand::{SeedableRng, Rng};

pub struct MultiDist {
    pub ballots: Ballot,
    rng: StdRng,
    weight_power: f64
}

impl MultiDist {
    pub fn new(ballots: &Ballot, random_seed: u64, weight_power: f64) -> MultiDist {
        MultiDist {
            ballots: ballots.clone(),
            rng: StdRng::seed_from_u64(random_seed),
            weight_power
        }
    }

    fn do_random_move(&mut self, schedule: &mut Vec<Vec<Student>>) {
        crate::log_debug!("Picking student from a random house to move", "random_move");
        let (house_index, student_index) = self.pick_student_to_move(schedule);
        let (house_index_2, student_index_2) = self.pick_move_location(house_index, student_index, schedule);
        let temp_student = schedule[house_index_2][student_index_2].clone();
        schedule[house_index_2][student_index_2] = schedule[house_index][student_index].clone();
        schedule[house_index][student_index] = temp_student;
        crate::log_debug!("Move success", "random_move");
    }

    fn pick_student_to_move(&mut self, schedule: &mut Vec<Vec<Student>>) -> (usize, usize) {
        let weight_power = self.weight_power;

        /* Pick house to move student from */
        let house_index = Self::pick_from_distribution(
            &schedule, |item, index| {
                Self::house_weight(item, index).powf(weight_power)
            },
            |item, weight, index| {
                crate::log_trace!(format!("[house] House {} has a total weight of {}", index, weight), "random_move")
            }, &mut self.rng);

        crate::log_debug!(format!("[house] ID {} ({}) was selected", house_index, self.ballots.houses[house_index].name), "random_move");

        /* Pick student in house */
        let student_index = Self::pick_from_distribution(
            &schedule[house_index],
            |item, index| {
                Self::student_inverse_weight(item, house_index).powf(weight_power)
            },
            |item, weight, index| {
                crate::log_trace!(format!("[student] {} has a total weight of {}", item.name, weight), "random_move")
            }, &mut self.rng);

        crate::log_debug!(format!("[student] {} was selected", schedule[house_index][student_index].name), "random_move");
        (house_index, student_index)
    }

    fn pick_move_location(&mut self, house_index: usize, student_index: usize, schedule: &mut Vec<Vec<Student>>) -> (usize, usize) {
        let student = &schedule[house_index][student_index];
        let weight_power = self.weight_power;

        let move_house_weight = |item: &Vec<Student>, index: usize| -> f64 {
            if index == house_index { return 0f64 }
            student.ballot[index].powf(weight_power)
        };

        let house_index_2 = Self::pick_from_distribution(
            schedule, move_house_weight,
            |item, weight, index| {
                crate::log_trace!(format!("[house-2] House {} has a weight of {}", index, weight), "random_move")
            }, &mut self.rng);

        crate::log_debug!(format!("[house-2] ID {} ({}) was selected", house_index_2, self.ballots.houses[house_index_2].name), "random_move");

        let move_student_weight = |item: &Student, index: usize| -> f64 {
            item.ballot[house_index].powf(weight_power)
        };

        let student_index_2 = Self::pick_from_distribution(
            &schedule[house_index_2], move_student_weight,
            |item, weight, index| {
                crate::log_trace!(format!("[student-2] {} has a weight of {}", item.name, weight), "random_move")
            }, &mut self.rng);

        crate::log_debug!(format!("[student-2] {} was selected", schedule[house_index_2][student_index_2].name), "random_move");

        (house_index_2, student_index_2)
    }

    /* Helper Functions */
    fn pick_from_distribution<T, F: Fn(&T, usize) -> f64>(set: &Vec<T>, weight: F, log_weight: fn(&T, f64, usize), rng: &mut StdRng) -> usize {
        let mut weights = vec![0.0; set.len()];

        for i in 0..set.len() {
            weights[i] = weight(&set[i], i);
            log_weight(&set[i], weights[i], i);
        }

        let mut dist = WeightedIndex::new(&weights).unwrap();
        rng.sample(&dist)
    }

    /* Weight Functions */
    fn student_inverse_weight(student: &Student, house_id: usize) -> f64 {
        1.0 - student.ballot[house_id] / student.ballot_sum
    }

    fn student_weight(student: &Student, house_id: usize) -> f64 {
        student.ballot[house_id] / student.ballot_sum
    }

    fn house_weight(house: &Vec<Student>, house_id: usize) -> f64 {
        let mut sum = 0f64;
        for student in house { sum += Self::student_inverse_weight(student, house_id); }
        sum
    }
}

impl Optimizer for MultiDist {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut schedule = generate_random_allocation(&self.ballots, 0);
        for _ in 0..rounds {
            self.do_random_move(&mut schedule);
        }
        return schedule;
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}
