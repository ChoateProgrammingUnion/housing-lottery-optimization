pub mod mcmc_naive;
pub mod minimax;
pub mod minimax_swap;
pub mod minimax_friends;

use optimizers::Optimizer;
use rand::{thread_rng, Rng};
use ballot::Student;

#[derive(Debug, Clone)]
pub(self) struct Proposal {
    pub(self) student_location: (usize, usize),
    pub(self) proposed_house: usize
}

pub(self) trait MCMCOptimizer: Optimizer {
    // An acceptance function takes in a particular location of the student (house, student) and the new house and returns a probability between 0-1 of acceptance.
    // 1 means a 100% probability of accepting
    fn acceptance(&self, schedule: &Vec<Vec<Student>>, proposal: Proposal) -> f64;
    // A proposal function samples from all the house-student pairs and returns a students random change ((house, student), new_house).

    fn propose(&self, schedule: &Vec<Vec<Student>>) -> Proposal;

    fn gen_bool(&self, prob: f64) -> bool {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: bool = rng.gen_bool(prob);
        return dice;
    }

    fn gen_range(&self, min: usize, max: usize) -> usize {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let mut rng = thread_rng();
        let dice: usize = rng.gen_range(min, max);

        return dice;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use ballot::Ballot;

    #[test]
    fn test_mcmc_swap() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut naive = optimizers::mcmcswap::mcmc_swap::MCMCSWAP::new(&input_ballot);

        assert!(optimizers::validate_ballot(&input_ballot, naive.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, naive.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, naive.optimize(100)));
    }

    #[test]
    fn test_minimax() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut minimax = optimizers::mcmc::minimax::Minimax::new(&input_ballot);

        assert!(optimizers::validate_ballot(&input_ballot, minimax.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax.optimize(100)));
    }

    #[test]
    fn test_deans_algo(){
        let input_ballot = input::load_input(ballot::normalize);

        let mut dean = optimizers::deans_algorithm::DeansAlgorithm::new(&input_ballot);

        assert!(optimizers::validate_ballot(&input_ballot, dean.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, dean.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, dean.optimize(100)));
    }

    #[test]
    fn test_multi_dist(){
        let input_ballot = input::load_input(ballot::normalize);

        let mut multi = optimizers::multi_dist::MultiDist::new(&input_ballot, 0);

        assert!(optimizers::validate_ballot(&input_ballot, multi.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, multi.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, multi.optimize(100)));
    }

    #[test]
    fn test_minimax_swap() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut minimax_swap = optimizers::mcmc::minimax_swap::MinimaxSwap::new(&input_ballot);

        assert!(optimizers::validate_ballot(&input_ballot, minimax_swap.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax_swap.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax_swap.optimize(100)));
    }

    #[test]
    fn test_minimax_friends() {
        let input_ballot = input::load_input(ballot::normalize);

        let mut minimax_friends = optimizers::mcmc::minimax_friends::MinimaxFriends::new(&input_ballot);

        assert!(optimizers::validate_ballot(&input_ballot, minimax_friends.optimize(0)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax_friends.optimize(1)));
        assert!(optimizers::validate_ballot(&input_ballot, minimax_friends.optimize(100)));
    }


}


