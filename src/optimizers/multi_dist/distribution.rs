use ballot::{Ballot, Student};

use super::super::rand::distributions::WeightedIndex;
use super::MultiDist;
use std::ptr;
use super::super::rand::rngs::StdRng;
use super::super::rand::Rng;
use std::ops::Index;

pub struct AllocatedStudent {
    pub name: String,
    pub ballot: Vec<f64>,
    pub friends: Vec<usize>,
    pub ballot_sum: f64,
    pub id: usize,
    pub location: (usize, usize)
}

impl AllocatedStudent {
    pub fn from_student(student: &Student, location: (usize, usize)) -> Self {
        let s = student.clone();
        Self {
            name: s.name,
            ballot: s.ballot,
            friends: s.friends,
            ballot_sum: s.ballot_sum,
            id: s.id,
            location
        }
    }

    pub fn to_student(&self) -> Student {
        Student {
            name: self.name.clone(),
            ballot: self.ballot.clone(),
            friends: self.friends.clone(),
            ballot_sum: self.ballot_sum,
            id: self.id
        }
    }
}

pub struct WeightedDistribution<I> {
    pub items: Vec<I>,
    pub weights: Vec<f64>,
    pub weight_sum: f64,
    pub distribution: WeightedIndex<f64>,
    pub weight_function: fn(index: usize, item: &I) -> f64
}

impl<I> WeightedDistribution<I> {
    pub fn new(items: Vec<I>, weight_function: fn(index: usize, item: &I) -> f64) -> Self {
        let weights: Vec<f64> = items.iter().enumerate().map(|x| {
            weight_function(x.0, x.1)
        }).collect();
        let distribution = WeightedIndex::new(&weights).unwrap();
        let weight_sum = weights.iter().sum();
        Self {
            items,
            weights,
            weight_sum,
            distribution,
            weight_function
        }
    }

    pub fn sample(&self, rng: &mut StdRng) -> usize {
        rng.sample(&self.distribution)
    }

    pub fn update_item(&mut self, index: usize) {
        self.weight_sum -= self.weights[index];
        let new_weight = (self.weight_function)(index, &self.items[index]);
        self.weights[index] = new_weight;
        self.weight_sum += self.weights[index];
        self.distribution.update_weights(&[(index, &new_weight)]);
    }

    pub fn access_mut<F: Fn(&mut I)>(&mut self, index: usize, f: F) {
        f(&mut self.items[index]);
        self.update_item(index)
    }

    pub fn swap(vec: &mut Vec<WeightedDistribution<I>>, a_loc: usize, a_idx: usize, b_loc: usize, b_idx: usize) {
        unsafe {
            let pa: *mut I = &mut vec[a_loc].items[a_idx];
            let pb: *mut I = &mut vec[b_loc].items[b_idx];
            ptr::swap(pa, pb);
        }

        vec[a_loc].update_item(a_idx);
        vec[b_loc].update_item(b_idx);
    }
}

impl<I> std::ops::Index<usize> for WeightedDistribution<I> {
    type Output = I;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<I> std::ops::IndexMut<usize> for WeightedDistribution<I> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<I: std::ops::Index<usize>> std::ops::Index<(usize, usize)> for WeightedDistribution<I> {
    type Output = I::Output;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl<I: std::ops::IndexMut<usize>> std::ops::IndexMut<(usize, usize)> for WeightedDistribution<I> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

pub type DistHouse = WeightedDistribution<AllocatedStudent>;
pub type DistAllocations = WeightedDistribution<DistHouse>;
