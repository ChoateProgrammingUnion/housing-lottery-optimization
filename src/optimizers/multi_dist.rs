mod distribution;

use ballot::{Ballot, Student};
use optimizers::{Optimizer, generate_random_allocation};

use super::rand::rngs::StdRng;
use super::rand::distributions::WeightedIndex;
use super::rand::{SeedableRng, Rng};
use std::ptr;
use std::env::Args;
use std::ops::Index;
use optimizers::multi_dist::distribution::{DistAllocations, WeightedDistribution, DistHouse, AllocatedStudent};

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

    fn do_random_move(&mut self, allocations: &mut DistAllocations) {
        crate::log_debug!("Picking student from a random house to move", "random_move");
        let loc_a = self.pick_student_to_move(allocations);
        let loc_b = self.pick_move_location(loc_a, allocations);
        Self::swap_students(allocations, loc_a, loc_b);
        crate::log_debug!("Move success", "random_move");
    }
    
    fn swap_students(allocations: &mut DistAllocations, loc_a: (usize, usize), loc_b: (usize, usize)) {
        DistHouse::swap(&mut allocations.items, loc_a.0, loc_a.1, loc_b.0, loc_b.1);
        allocations[loc_a].location = loc_a;
        allocations[loc_b].location = loc_b;
        allocations.update_item(loc_a.0);
        allocations.update_item(loc_b.0);
    }

    fn pick_student_to_move(&mut self, allocations: &DistAllocations) -> (usize, usize) {
        /* Pick house to move student from */
        let house_index = self.rng.sample(&allocations.distribution);

        crate::log_debug!(format!("[house] ID {} ({}) was selected", house_index, self.ballots.houses[house_index].name), "random_move");

        /* Pick student in house */
        let student_index = allocations[house_index].sample(&mut self.rng);

        crate::log_debug!(format!("[student] {} was selected", allocations[house_index][student_index].name), "random_move");
        (house_index, student_index)
    }

    fn pick_move_location(&mut self, student_loc: (usize, usize), allocations: &DistAllocations) -> (usize, usize) {
        let student = &allocations[student_loc];

        let move_house_weight = |item: &DistHouse, index: usize| -> f64 {
            if index == student_loc.0 { return 0f64 }
            student.ballot[index].powf(10.0)
        };

        let house_index_2 = Self::pick_from_distribution(
            &allocations.items, move_house_weight,
            |item, weight, index| {
                crate::log_trace!(format!("[house-2] House {} has a weight of {}", index, weight), "random_move")
            }, &mut self.rng);

        crate::log_debug!(format!("[house-2] ID {} ({}) was selected", house_index_2, self.ballots.houses[house_index_2].name), "random_move");

        let move_student_weight = |item: &AllocatedStudent, index: usize| -> f64 {
            let mut a = item.ballot[student_loc.0] - item.ballot[house_index_2];
            if a < 0.0 { a = 0.0 }
            (a + 0.0001).powf(10.0)
        };

        let student_index_2 = Self::pick_from_distribution(
            &allocations[house_index_2].items, move_student_weight,
            |item, weight, index| {
                crate::log_trace!(format!("[student-2] {} has a weight of {}", item.name, weight), "random_move")
            }, &mut self.rng);

        crate::log_debug!(format!("[student-2] {} was selected", allocations[house_index_2][student_index_2].name), "random_move");

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
    fn student_inverse_weight(student: &AllocatedStudent) -> f64 {
        1.0 - student.ballot[student.location.0] / student.ballot_sum
    }

    fn student_weight(student: &AllocatedStudent, house_id: usize) -> f64 {
        student.ballot[house_id] / student.ballot_sum
    }
}

impl Optimizer for MultiDist {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut allocation = generate_random_allocation(&self.ballots, 0);

        let mut weighted_houses = vec![];

        for (index, house) in allocation.iter().enumerate() {
            let allocated_students = house.iter().enumerate().map(|x| {
                AllocatedStudent::from_student(x.1, (index, x.0))
            }).collect();
            weighted_houses.push(DistHouse::new(allocated_students, |index, student| {
                Self::student_inverse_weight(student)
            }))
        }

        let mut weighted_allocations = DistAllocations::new(weighted_houses, |index, house| {
            house.weight_sum
        });

        for _ in 0..rounds {
            self.do_random_move(&mut weighted_allocations);
        }

        return weighted_allocations.items.iter().map(|x| {
            x.items.iter().map(|y| {
                y.to_student()
            }).collect()
        }).collect();
    }

    fn reseed(&mut self, new_seed: u64) {
        self.rng = StdRng::seed_from_u64(new_seed);
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}