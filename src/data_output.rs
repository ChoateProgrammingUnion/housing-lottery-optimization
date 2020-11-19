use crate::ballot::{Ballot, Student};

use std::fs::File;
use std::io::Write;
use std::time::Duration;

// Struct to hold trial results and output distributions
pub(self) struct TrialData {
    pub(self) choice_nums: Vec<f64>, // Choice distribution
    pub(self) friend_nums: Vec<f64>, // Friend distribution
    pub(self) run_time_nanos: u128, // Run time, in nanoseconds
}

impl TrialData {
    pub(self) fn new(
        choice_nums: Vec<f64>,
        friend_nums: Vec<f64>,
        run_time_nanos: u128,
    ) -> TrialData {
        Self {
            choice_nums,
            friend_nums,
            run_time_nanos,
        }
    }
}

fn get_trial_data(data: &Vec<Vec<Student>>, num_houses: usize, run_time: &Duration) -> TrialData {
    let mut choice_nums: Vec<f64> = vec![0.0; num_houses];
    let mut friend_nums: Vec<f64> = vec![];

    for house_num in 0..data.len() {
        for student in &data[house_num] {
            let house_score = student.ballot[house_num];
            let mut choice_num = 1;

            for n in &student.ballot {
                if n > &house_score {
                    choice_num += 1
                }
            }

            let mut friend_count: usize = 0;
            for other_student in &data[house_num] {
                if student.id == other_student.id {
                    break;
                }
                if student.friends.contains(&other_student.id) {
                    friend_count += 1;
                }
            }
            while friend_count >= friend_nums.len() {
                friend_nums.push(0.0)
            }
            friend_nums[friend_count] += 1.0;

            choice_nums[choice_num - 1] += 1.0;
        }
    }

    TrialData::new(choice_nums, friend_nums, run_time.as_nanos())
}

fn average_data(data: &Vec<TrialData>) -> TrialData {
    let mut average_choice_nums: Vec<f64> = data[0].choice_nums.clone();
    let mut average_friend_nums: Vec<f64> = data[0].friend_nums.clone();
    let mut average_run_time: u128 = data[0].run_time_nanos;

    for i in 1..data.len() {
        assert_eq!(average_choice_nums.len(), data[i].choice_nums.len());

        for j in 0..average_choice_nums.len() {
            average_choice_nums[j] += data[i].choice_nums[j];
        }

        while data[i].friend_nums.len() > average_friend_nums.len() {
            average_friend_nums.push(0.0);
        }
        for j in 0..data[i].friend_nums.len() {
            average_friend_nums[j] += data[i].friend_nums[j];
        }

        average_run_time += data[i].run_time_nanos;
    }

    average_run_time /= data.len() as u128;

    for i in 0..average_choice_nums.len() {
        average_choice_nums[i] /= data.len() as f64;
    }

    for i in 0..average_friend_nums.len() {
        average_friend_nums[i] /= data.len() as f64;
    }

    TrialData::new(average_choice_nums, average_friend_nums, average_run_time)
}

pub fn write_output(
    allocations: &Vec<Vec<Vec<Student>>>,
    ballot: &Ballot,
    run_times: &Vec<Duration>,
    data_file: &mut File,
    algo: String,
) {
    assert_eq!(allocations.len(), run_times.len());

    // Get data
    let mut data: Vec<TrialData> = vec![];
    for i in 0..allocations.len() {
        data.push(get_trial_data(
            &allocations[i],
            ballot.houses.len(),
            &run_times[i],
        ))
    }

    // Get average data
    let average_data = average_data(&data);

    // Write elapsed time
    let mut yaml_string = format!("run_time_nanos: {}\n", average_data.run_time_nanos);

    // Write Choice Numbers
    yaml_string += "    choice_distribution:\n";
    for i in 0..average_data.choice_nums.len() {
        yaml_string += &*format!("      - {}: {}\n", i + 1, average_data.choice_nums[i]);
    }

    // Write Friend Numbers
    yaml_string += "    friend_distribution:\n";
    for i in 0..average_data.friend_nums.len() {
        yaml_string += &*format!("      - {}: {}\n", i, average_data.friend_nums[i]);
    }

    data_file
        .write(format!("\n  - name: {}\n    {}", algo, yaml_string).as_ref())
        .unwrap();
}
