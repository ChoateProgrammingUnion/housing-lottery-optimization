use optimizers::Optimizer;
use ballot::{Ballot, Student};
use std;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Ordering, Eq};

use petgraph::stable_graph::{StableGraph, NodeIndex};

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

// Credit: code modified from rust docs
#[derive(Copy, Clone, PartialEq)]
pub struct OrderedNode {
    node: NodeIndex,
    distance: f64,
}

impl OrderedNode {
    pub fn new(node: NodeIndex, distance: f64) -> Self{
        Self {
            node: node,
            distance: distance
        }
    }
}

// impl Eq for OrderedNode {
//     fn eq(&self, other: &Self) -> bool {
//         if (self.distance - other.distance).abs() < 1e-10 {
//             return true
//         } else {
//             return false
//         }
//     }
// }

impl Eq for OrderedNode {}

impl Ord for OrderedNode {
    fn cmp(&self, other: &OrderedNode) -> Ordering {
        let diff = self.distance - other.distance;
        if diff.abs() < 1e-10 {
            Ordering::Equal
        } else if diff > 1e-10 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for OrderedNode {
    fn partial_cmp(&self, other: &OrderedNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn k_nearest(contraction_graph: &mut StableGraph<Node, f64>, start: NodeIndex, k: usize, exclude: &HashMap<NodeIndex, bool>) -> Vec<OrderedNode> {
    // Give the k nearest nodes from the starting node, excluding a list of nodes.
    // If there are no such k nodes, return the first n up to k nodes.
    // let mut nearest: Vec<&'a NodeIndex> = Vec::<NodeIndex>::new();
    let mut nearest = Vec::<NodeIndex>::new();
    let mut result = Vec::<OrderedNode>::new();
    let mut heap = BinaryHeap::<OrderedNode>::new();

    let mut counter: usize = 0;

    heap.push(OrderedNode{node: start.clone(), distance: 0.0});

    while result.len() <= k {
        if !heap.is_empty() {
            let mut current_node = heap.peek().unwrap();
            let mut current_distance = current_node.distance.clone();
            let mut walker = contraction_graph.neighbors(current_node.node).detach();

            while let Some((edge, neighbor)) = walker.next(&contraction_graph) {
                if !exclude.contains_key(&neighbor) {
                    heap.push(OrderedNode::new(neighbor, current_distance + contraction_graph.edge_weight(edge).unwrap())); // might need orginal graph
                    contraction_graph.remove_node(neighbor);
                }
            }

            result.push(heap.pop().unwrap())
        }
        // else {
        //     if result.len() <= k { // debug
        //         panic!();
        //     }
        // }
    }

    result.pop(); // remove starting point
    return result
}

#[derive(Clone)]
pub struct NetworkOptimizer {
    ballots: Ballot,
    graph: StableGraph::<Node, f64>,
    house_nodes: Vec<NodeIndex>
}

impl NetworkOptimizer {
    pub fn new(ballots: &Ballot, friend_ratio: f64) -> Self {
        let mut graph = StableGraph::<Node, f64>::new();

        let mut house_nodes = Vec::<NodeIndex>::new();
        for house in &ballots.houses {
            let mut house_node = Node::new(house.name.clone(), None).clone();
            house_nodes.push(graph.add_node(house_node));
        }

        let mut student_nodes = Vec::<NodeIndex>::new();
        for (count, student) in ballots.students.iter().enumerate() {
            let mut student_node = Node::new(student.name.clone(), Some(student.clone()));
            student_nodes.push(graph.add_node(student_node));

            for (house_num, housing_pref) in student.ballot.iter().enumerate() {
                graph.add_edge(house_nodes[house_num], *student_nodes.last().unwrap(), friend_ratio*(1.0/housing_pref));
            }

            for friend_pref in &student.friends { // here we assume that it is reciprocated
                if friend_pref < &count { // we've already added the student
                    let mut friend_node = student_nodes[*friend_pref];
                    graph.add_edge(*student_nodes.last().unwrap(), friend_node, 1.0);
                } // we have not added the student, so we skip
                // Since all friendships must be reciprocated, we'll see this friendship later
            }
            
        }

        Self {
            ballots: ballots.clone(),
            graph: graph,
            house_nodes: house_nodes
        }
    }

    pub fn optimize(self) -> Vec<Vec<Student>> {
        let mut contraction_graph = self.graph.clone(); // for leaf contraction
        let mut schedule: Vec<Vec<Student>> = vec![vec![]; self.ballots.houses.len()];
        let mut exclude: HashMap<NodeIndex, bool> = HashMap::new();

        for house in &self.house_nodes {
            exclude.insert(*house, true);
        }
            

        let mut counter = 0;
        for house in self.house_nodes {
            let mut max_cap = self.ballots.houses[counter].capacity.clone();

            let mut student_nodes = k_nearest(&mut contraction_graph.clone(), house, max_cap, &exclude);

            for student in student_nodes {
                schedule[counter].push(contraction_graph.remove_node(student.node).unwrap().student.unwrap());
            }

            counter += 1;
        }

        return schedule
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::collections::{HashMap};

    #[test]
    fn test_graph_init() {
        let ballot = input::load_input(ballot::normalize);
        let graph = optimizers::network::NetworkOptimizer::new(&ballot, 10.0);
    }

    #[test]
    fn test_graph_nearest() {
        let ballot = input::load_input(ballot::normalize);
        let mut exclude: HashMap<optimizers::network::NodeIndex, bool> = HashMap::new();
        let mut graph = optimizers::network::NetworkOptimizer::new(&ballot, 10.0);

        let test = graph.graph.node_indices().next();
        assert_eq!(optimizers::network::k_nearest(&mut graph.graph.clone(), test.unwrap(), 3, &exclude).len(), 3);
    }

    #[test]
    fn test_optimize() {
        let ballot = input::load_input(ballot::normalize);
        let mut exclude: HashMap<optimizers::network::NodeIndex, bool> = HashMap::new();
        let mut graph = optimizers::network::NetworkOptimizer::new(&ballot, 10.0);

        graph.optimize();
    }

    // #[test]
    // #[should_panic]
    // fn test_graph_nearest_panic() {
    //     let ballot = input::load_input(ballot::normalize);
    //     let mut graph = network::NetworkOptimizer::new(&ballot, 10.0);

    //     let test = graph.graph.node_indices().next();
    //     graph.k_nearest(test.unwrap(), 1000);
    // }

    #[test]
    fn test_mcmc_naive() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut naive = optimizers::network::NetworkOptimizer::new(&input_ballot, 10.0);
        assert!(optimizers::validate_ballot(&input_ballot, naive.optimize()));
    }
}
