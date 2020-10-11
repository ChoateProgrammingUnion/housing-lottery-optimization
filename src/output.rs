use crate::ballot::{Ballot, Student};

use std::fs::File;
use std::io::Write;

pub fn write_output(allocations: &Vec<Vec<Student>>, ballot: &Ballot) {
    // Open output file
    let mut output_file = File::create("output.yaml").expect("file creation failed");

    let mut yaml_string = "students:\n".to_string();

    for house_num in 0..allocations.len() {
        for student in &allocations[house_num] {
            yaml_string += &*format!("  - name: {}\n    assignment: {}\n",
                                     &student.name, &ballot.houses[house_num].name)
        }
    }

    yaml_string += "\n\nhouses:\n";

    for house_num in 0..allocations.len() {
        yaml_string += &*format!("  - {}:\n", &ballot.houses[house_num].name);

        for student in &allocations[house_num] {
            yaml_string += &*format!("     - {}\n", &student.name)
        }
    }

    output_file.write(yaml_string.as_ref()).unwrap();
}
