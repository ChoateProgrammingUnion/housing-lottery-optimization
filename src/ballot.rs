#[derive(Debug, Clone)]
pub struct Student {  // Perhaps rename because a double would count as 1 student
    pub name: String,
    pub ballot: Vec<f64>,
    pub friends: Vec<String>,
    pub ballot_sum: f64
}

impl Student {
    pub fn new(name: String, num_houses: usize) -> Self {
        Self {
            name,
            ballot: vec![0.0; num_houses],
            friends: vec![String::new()],
            ballot_sum: 0.0
        }
    }
}

#[derive(Debug, Clone)]
pub struct House {
    pub name: String,
    pub capacity: usize
}

impl House {
    pub fn new(name: String, capacity: usize) -> Self {
        Self {
            name,
            capacity
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ballot {
    pub students: Vec<Student>,
    pub houses: Vec<House>
}

impl Ballot {
    pub fn new() -> Self {
        Self {
            students: vec![],
            houses: vec![]
        }
    }
}

// scaled the max rating to 1, and everything else scaled proportionally
pub fn scaled(student: Student) -> Student {
    // find max
    let mut max: f64 = 0.0;
    for elm in &student.ballot{
        if elm>&max {
            max = *elm;
        }
    }

    // scale ballot to maximum
    let mut scaled = Student::new(student.name, 0);
    for i in 0..student.ballot.len(){
        scaled.ballot[i] = student.ballot[i]/max;
    }

    scaled
}

// normalize the sum of the ratings to 1
pub fn normalize(student: Student) -> Student {
    // finds sum
    let sum: f64 = student.ballot.iter().sum();

    // normalize ballot to sum
    let mut normalized = Student::new(student.name, 0);
    for i in 0..student.ballot.len(){
        normalized.ballot[i] = student.ballot[i]/sum;
    }

    normalized
}