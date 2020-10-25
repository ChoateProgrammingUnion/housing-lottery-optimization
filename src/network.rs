use optimizers::Optimizer;
use ballot::{Ballot, Student};
use std;

use petgraph::graph::{Graph, NodeIndex};

pub struct NetworkOptimizer {
    ballots: Ballot,
    graph: Graph::<Node, f64>
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    // pub student: Option<Student>,
    pub student: std::option::Option<Student>,
}

impl Node {
    pub fn new(name: String, student: std::option::Option<Student>) -> Self{
        Self {
            name: name,
            student: student
        }
    }
}

impl NetworkOptimizer {
    pub fn new(ballots: &Ballot) -> Self {
        let mut null_graph = Graph::<Node, f64>::new();
        Self {
            ballots: ballots.clone(),
            graph: null_graph
        }
    }

    pub fn instantiate(mut self, friend_ratio: f64) {
        // friend_weight is the fixed weight assigned to each friendship pair

        let mut house_nodes = Vec::<NodeIndex>::new();
        for house in &self.ballots.houses {
            let mut house_node = Node::new(house.name.clone(), None).clone();
            house_nodes.push(self.graph.add_node(house_node));
        }

        let mut student_nodes = Vec::<NodeIndex>::new();
        for (count, student) in self.ballots.students.iter().enumerate() {
            let mut student_node = Node::new(student.name.clone(), Some(student.clone()));
            student_nodes.push(self.graph.add_node(student_node));

            for (house_num, housing_pref) in student.ballot.iter().enumerate() {
                self.graph.add_edge(house_nodes[house_num], *student_nodes.last().unwrap(), friend_ratio*(1.0/housing_pref));
            }

            for friend_pref in &student.friends { // here we assume that it is reciprocated
                if friend_pref < &count { // we've already added the student
                    let mut friend_node = student_nodes[*friend_pref];
                    self.graph.add_edge(*student_nodes.last().unwrap(), friend_node, 1.0);
                } // we have not added the student, so we skip
                // Since all friendships must be reciprocated, we'll see this friendship later
            }
            
        }
        
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_graph_init() {
        let ballot = input::load_input(ballot::normalize);
        let graph = network::NetworkOptimizer::new(&ballot);
        graph.instantiate(10.0);
    }

    #[test]
    fn test_graph_instantiate() {
        let ballot = input::load_input(ballot::identity);
    }
}
