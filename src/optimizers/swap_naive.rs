use ballot::{Ballot, Student};
use optimizers::Optimizer;

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[derive(Clone)]
pub struct SwapNaive {
    pub ballots: Ballot,
    rng: StdRng
}

impl SwapNaive {
    #[allow(dead_code)]
    pub fn new(ballots: &Ballot, random_seed: u64) -> SwapNaive {
        SwapNaive {
            ballots: ballots.clone(),
            rng: StdRng::seed_from_u64(random_seed)
        }
    }

    pub fn all_first_choice(&self) -> Vec<Vec<Student>> {
        let mut allocations: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];

        for student in &self.ballots.students {
            let mut first_choice_index: usize = 0;
            let mut first_choice_weight: f64 = f64::MIN;
            let mut found_first_choice: bool = false;

            for (index, weight) in student.ballot.iter().enumerate() {
                if &first_choice_weight < weight {
                    first_choice_index = index;
                    first_choice_weight = *weight;
                    found_first_choice = true;
                }
            }

            if !found_first_choice {
                panic!("could not find first choice!")
            }

            allocations[first_choice_index].push(student.clone());
        }

        allocations
    }

    pub fn ensure_houses_at_capacity(&mut self, allocations: &mut Vec<Vec<Student>>) {
        let num_houses = allocations.len();
        for house_id in 0..num_houses {
            while allocations[house_id].len() > self.ballots.houses[house_id].capacity {
                let house_len = allocations[house_id].len();
                let student_id = self.rng.gen_range(0, house_len);
                let student = &allocations[house_id][student_id];

                let mut highest_open_house_index: usize = 0;
                let mut highest_open_house_weight: f64 = f64::MIN;
                let mut found_open_house: bool = false;

                for house_id_2 in 0..num_houses {
                    if house_id == house_id_2 { continue }

                    let house_2 = &allocations[house_id_2];

                    if house_2.len() < self.ballots.houses[house_id_2].capacity {
                        if highest_open_house_weight < student.ballot[house_id_2] {
                            highest_open_house_index = house_id_2;
                            highest_open_house_weight = student.ballot[house_id_2];
                            found_open_house = true;
                        }
                    }
                }

                if !found_open_house {
                    panic!("could not find open house!")
                }

                let student = allocations[house_id].remove(student_id);
                allocations[highest_open_house_index].push(student);
            }
        }
    }

    pub fn sort_allocations(allocations: &mut Vec<Vec<Student>>) {
        for house_id in 0..allocations.len() {
            allocations[house_id].sort_by(|a, b| a.ballot[house_id].partial_cmp(&b.ballot[house_id]).unwrap().reverse());

            crate::log_debug!(format!("Sorted house {}", house_id), "sort");

            for student in &allocations[house_id] {
                crate::log_debug!(format!("{} - {}", student.name, student.ballot[house_id]), "sort");
            }
        }
    }

    pub fn do_swap(&mut self, allocations: &mut Vec<Vec<Student>>, last_swap: Option<(usize, usize)>) -> Option<(usize, usize)> {
        // Find least satisfied student
        let mut least_satisfied_weight: f64 = f64::MAX;
        let mut least_satisfied_index: usize = 0;
        for house_id in 0..allocations.len() {
            let student = allocations[house_id].last().unwrap();
            if student.ballot[house_id] < least_satisfied_weight {
                least_satisfied_weight = student.ballot[house_id];
                least_satisfied_index = house_id;
            }
        }
        let least_satisfied_student = allocations[least_satisfied_index].last().unwrap();

        // Find swap that maximizes net preference increase
        let mut highest_weight_increase: f64 = f64::MIN;
        let mut swap_position: (usize, usize) = (0, 0);
        for house_id in 0..allocations.len() {
            if house_id == least_satisfied_index { continue }
            for (student_loc, student) in allocations[house_id].iter().enumerate() {
                let current_net_weight = least_satisfied_student.ballot[least_satisfied_index] + student.ballot[house_id];
                let swap_net_weight = least_satisfied_student.ballot[house_id] + student.ballot[least_satisfied_index];
                let weight_diff = swap_net_weight - current_net_weight;

                if weight_diff > highest_weight_increase {
                    highest_weight_increase = weight_diff;
                    swap_position = (house_id, student_loc);
                }
            }
        }

        crate::log_debug!(format!("Swapping {} and {} for a net weight diff of {}", least_satisfied_student.name, allocations[swap_position.0][swap_position.1].name, highest_weight_increase), "swap");

        let a_loc = least_satisfied_index;
        let a_idx = allocations[a_loc].len() - 1;
        let (b_loc, b_idx) = swap_position;

        match last_swap {
            Some((a, b)) => {
                if (allocations[a_loc][a_idx].id == a && allocations[b_loc][b_idx].id == b) ||
                    (allocations[a_loc][a_idx].id == b && allocations[b_loc][b_idx].id == a) {
                    return None;
                }
            }
            _ => {}
        }

        unsafe {
            let pa: *mut Student = &mut allocations[a_loc][a_idx];
            let pb: *mut Student = &mut allocations[b_loc][b_idx];

            std::ptr::swap(pa, pb);
        }

        allocations[a_loc].sort_by(|a, b| a.ballot[a_loc].partial_cmp(&b.ballot[a_loc]).unwrap().reverse());
        allocations[b_loc].sort_by(|a, b| a.ballot[b_loc].partial_cmp(&b.ballot[b_loc]).unwrap().reverse());

        Some((allocations[a_loc][a_idx].id, allocations[b_loc][b_idx].id))
    }

    pub fn score(state: &Vec<Vec<Student>>) -> f64 {
        let mut total = 0f64;

        for (house_id, house) in state.iter().enumerate() {
            for student in house {
                total += student.ballot[house_id];
            }
        }

        total
    }

    pub fn local_max(&mut self, start_state: &Vec<Vec<Student>>) -> (f64, Vec<Vec<Student>>) {
        let mut end_state = start_state.clone();

        let mut last_swap: Option<(usize, usize)> = None;
        for _ in 0..1000 {
            last_swap = self.do_swap(&mut end_state, last_swap);
            match last_swap {
                None => {
                    break;
                }
                _ => {}
            }
        }

        (Self::score(&end_state), end_state)
    }

    pub fn mix(&mut self, allocations: &mut Vec<Vec<Student>>, n: usize) {
        for _ in 0..n {
            let a_loc = self.rng.gen_range(0, allocations.len());
            let a_idx = self.rng.gen_range(0, allocations[a_loc].len());
            let b_loc = self.rng.gen_range(0, allocations.len());
            let b_idx = self.rng.gen_range(0, allocations[b_loc].len());

            unsafe {
                let pa: *mut Student = &mut allocations[a_loc][a_idx];
                let pb: *mut Student = &mut allocations[b_loc][b_idx];

                std::ptr::swap(pa, pb);
            }
        }

        Self::sort_allocations(allocations);
    }
}

impl Optimizer for SwapNaive {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut allocations = self.all_first_choice();

        self.ensure_houses_at_capacity(&mut allocations);
        Self::sort_allocations(&mut allocations);

        let mut state = allocations.clone();
        let mut best_state: Vec<Vec<Student>> = vec![];
        let mut best_score = f64::MIN;

        for _ in 0..rounds {
            let (score, new_state) = self.local_max(&state);

            if score > best_score {
                best_state = new_state.clone();
                best_score = score;
            }

            state = new_state;
            self.mix(&mut state, self.ballots.students.len() / 5);
        }

        best_state
    }

    fn reseed(&mut self, new_seed: u64) {
        self.rng = StdRng::seed_from_u64(new_seed);
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}