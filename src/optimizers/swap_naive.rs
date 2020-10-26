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
}

impl Optimizer for SwapNaive {
    fn optimize(&mut self, rounds: usize) -> Vec<Vec<Student>> {
        let mut allocations = self.all_first_choice();
        self.ensure_houses_at_capacity(&mut allocations);
        allocations
    }

    fn reseed(&mut self, new_seed: u64) {
        self.rng = StdRng::seed_from_u64(new_seed);
    }

    fn objective(&self) -> f64 {
        return 0.0;
    }
}