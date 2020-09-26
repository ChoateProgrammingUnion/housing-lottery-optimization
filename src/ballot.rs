#[derive(Debug, Clone)]
pub struct Student {  // Perhaps rename because a double would count as 1 student
    pub name: String,
    pub ballot: Vec<f64>,
}

impl Student {
    pub fn new(name: String, num_houses: usize) -> Self {
        Self {
            name,
            ballot: vec![0.0; num_houses]
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

#[derive(Debug)]
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

