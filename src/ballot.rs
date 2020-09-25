#[derive(Debug, Clone)]
pub struct Student {  // Perhaps rename because a double would count as 1 student
    pub name: String,
    pub ballot: Vec<u8>,
}

impl Student {
    pub fn new(name: String, num_houses: u8) -> Self {
        Self {
            name,
            ballot: vec![0; num_houses as usize]
        }
    }
}

#[derive(Debug, Clone)]
pub struct House {
    pub name: String,
    pub capacity: u8
}

impl House {
    pub fn new(name: String, capacity: u8) -> Self {
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