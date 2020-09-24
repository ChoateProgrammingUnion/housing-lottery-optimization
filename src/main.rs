mod ballot;

use crate::ballot::Student;

fn main() {
    let mut student = Student::new("Ethan Chapman", 10);

    println!("{:?}", student);
}
