extern crate yaml_rust;
use self::yaml_rust::YamlLoader;

use crate::ballot::{Ballot, Student, House};

use std::io::Read;
use std::collections::HashMap;

pub fn load_input(process: fn(Student) -> Student) -> Ballot {
    // Load file
    let mut input_file = std::fs::File::open("input.yaml").expect("yaml file not found");
    let mut input_str: String = String::new();
    input_file.read_to_string(&mut input_str).expect("input file read failed");
    let input = YamlLoader::load_from_str(&*input_str).expect("yaml failed to load");
    let houses = input[0]["houses"].clone().into_vec().expect("houses is not an array");
    let ballots = input[0]["ballots"].clone().into_vec().expect("ballots is not an array");

    // Hash map for changing house name into id
    let mut house_name_map: HashMap<String, f64> = HashMap::new();

    // Set up Ballot object
    let mut new_ballot = Ballot::new();
    let num_houses = houses.len();

    // Add houses
    for house in houses {
        let name = house["name"].as_str().expect("house name is not a string");
        let capacity = house["capacity"].as_i64().expect("house capacity is not an integer");

        house_name_map.insert(String::from(name), new_ballot.houses.len() as f64);

        new_ballot.houses.push(House::new(String::from(name), capacity as usize))
    }

    // Add ballots
    for ballot in ballots {
        let student_name = ballot["name"].as_str().expect("student name is not a string");
        let rankings = ballot["rankings"].clone().into_vec().expect("student rankings is not an array");

        let mut student = Student::new(String::from(student_name), num_houses);

        for ranking in rankings {
            let house_name = ranking["name"].as_str().expect("house name is not a string");
            let house_weight = ranking["weight"].as_f64().expect("house weight is not a float");
            let house_index = house_name_map[house_name];
            student.ballot[house_index as usize] = house_weight;
            student.ballot_sum += house_weight;
        }

        new_ballot.students.push(process(student));
    }

    new_ballot
}
