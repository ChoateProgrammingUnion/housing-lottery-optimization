#[derive(Debug)]
pub struct Student<'a> {  // Perhaps rename because a double would count as 1 student
    pub name: &'a str,
    pub ballot: Vec<u8>,
}

impl<'a> Student<'a> {
    pub fn new(name: &'a str, num_houses: u8) -> Self {
        Self {
            name,
            ballot: vec![0; num_houses as usize]
        }
    }
}

#[derive(Debug)]
pub struct House<'a> {
    pub name: &'a str,
    pub capacity: u8
}

impl<'a> House<'a> {
    pub fn new(name: &'a str, capacity: u8) -> Self {
        Self {
            name,
            capacity
        }
    }
}

#[derive(Debug)]
struct Ballot<'a> {
    pub students: Vec<Student<'a>>,
    pub houses: Vec<House<'a>>
}

impl<'a> Ballot<'a> {
    pub fn new() -> Self {
        Self {
            students: vec![],
            houses: vec![]
        }
    }
}