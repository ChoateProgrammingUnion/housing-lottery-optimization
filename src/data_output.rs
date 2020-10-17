use crate::ballot::{Ballot, Student};

use std::fs::File;
use std::io::Write;
use std::time::Duration;

pub fn write_output(allocations: &Vec<Vec<Student>>, ballot: &Ballot, optimized_time: &Duration) {
    // Open output file
    let mut data_file = File::create("data_output.yaml").expect("file creation failed");

    // Write elapsed time
    let mut yaml_string = format!("run_time_nanos: {}\n", optimized_time.as_nanos());

    // Find choice numbers
    let mut choice_nums: Vec<usize> = vec![0; ballot.houses.len()];

    for i in 0..allocations.len() {
        for student in &allocations[i] {
            let house_score = student.ballot[i];
            let mut choice_num = 1;

            for n in &student.ballot {
                if n > &house_score { choice_num += 1 }
            }

            choice_nums[choice_num - 1] += 1;
        }
    }

    // Write Choice Numbers
    yaml_string += "\nchoice_distribution:\n";
    for i in 0..choice_nums.len() {
        yaml_string += &*format!("  - {}: {}\n", i + 1, choice_nums[i]);
    }

    data_file.write(yaml_string.as_ref()).unwrap();
}
