mod ballot;
mod input;
mod output;
mod data_output;
mod optimizers;
mod logger;
use std::fs::File;
use std::io::Write;

use optimizers::Optimizer;

use std::time::{Instant, Duration};

extern crate log;
extern crate chrono;
extern crate array2d;
extern crate rand;
extern crate petgraph;

use log::LevelFilter;
use ballot::Student;

fn main() {

    // Change this to set the log level
    // LevelFilter::Off   - No logging (USE THIS FOR BENCHMARKS AS LOGS TAKE TIME TO PRINT)
    // LevelFilter::Error - Print errors (nonfatal errors that are logged)
    // LevelFilter::Info  - Print info messages (and errors)
    // LevelFilter::Debug - Print debug messages (and info, error)
    // LevelFilter::Trace - Print trace messages (and info, error, debug) (a lot of messages)
    logger::init(LevelFilter::Info);

    crate::log_info!("processing...", "input");
    let ballot = input::load_input(
        // ballot::identity
        // ballot::normalize
        ballot::scale
    );
    crate::log_info!("successfully processed", "input");

    let trials = input::load_trials();
    let mut data_file = File::create("data_output.yaml").expect("file creation failed");
    data_file.write("\nalgo:".as_ref());

    for trial in trials {
        let start_seed: u64 = 0;
        let trials: usize = 100;
        let rounds: usize = 10000;
        let threads: usize = 8;
        let mut results: Vec<Vec<Vec<Student>>> = vec![];
        let mut times: Vec<Duration> = vec![];
        let write_first_allocation_vector = false;


        let mut handles: Vec<std::thread::JoinHandle<(Vec<Duration>, Vec<Vec<Vec<Student>>>)>> = vec![];
        for t in 0..threads {
            let trial_name = trial.clone();
            let ballot_copy = ballot.clone();

            handles.push(std::thread::spawn(move || {
                let mut optimizer = select_optimizer(trial_name.as_str(), &ballot_copy);
                // let mut optimizer = new_optimizer;
                let mut durations: Vec<Duration> = vec![];
                let mut allocations: Vec<Vec<Vec<Student>>> = vec![];
                for trial_num in (t..trials).step_by(threads) {
                    let (optimized_time, result) = run_trial(trial_num, rounds, start_seed, &mut optimizer);
                    durations.push(optimized_time);
                    allocations.push(result);
                }
                (durations, allocations)
            }));
        }
        for h in handles {
            let (mut t, mut r) = h.join().unwrap();
            times.append(&mut t);
            results.append(&mut r);
        }

        crate::log_info!("writing", "output");
        if write_first_allocation_vector { output::write_output(&results[0], &ballot); }
        data_output::write_output(&results, &ballot, &times, &mut data_file, trial);
        crate::log_info!("finished", "output");
    }
}

fn select_optimizer(trial_name: &str, ballot: &ballot::Ballot) -> Box<dyn Optimizer>{
    if trial_name == "multi" {
        return Box::new(optimizers::multi_dist::MultiDist::new(ballot, 0));
    } else if trial_name == "minimax-friends" {
        return Box::new(optimizers::mcmc::minimax_friends::MinimaxFriends::new(ballot));
    } else if trial_name ==  "minimax" {
        return Box::new(optimizers::mcmc::minimax::Minimax::new(ballot));
    } else if trial_name == "deans" {
        return Box::new(optimizers::deans_algorithm::DeansAlgorithm::new(ballot));
    } else if trial_name == "network" {
        return Box::new(optimizers::network::NetworkOptimizer::new(ballot, 10.0, 10.0)); // use with normalize or scale; expects 0-1 range
    } else {
        return Box::new(optimizers::mcmc::mcmc_naive::MCMCNaive::new(ballot));
    }
}

// fn select_optimizer(trial_name: &str, ballot: &ballot::Ballot) -> Option<dyn Optimizer>{
//     if trial_name == "multi" {
//         return Some(optimizers::multi_dist::MultiDist::new(ballot, 0));
//     } else if trial_name == "swap" {
//         return Some(optimizers::mcmc::mcmc_swap::MCMCSWAP::new(ballot));
//     } else if trial_name ==  "minimax" {
//         return Some(optimizers::mcmc::minimax::Minimax::new(ballot));
//     } else if trial_name == "deans" {
//         return Some(optimizers::deans_algorithm::DeansAlgorithm::new(ballot));
//     } else if trial_name == "network" {
//         return Some(optimizers::network::NetworkOptimizer::new(ballot, 10.0, 10.0)); // use with normalize or scale; expects 0-1 range
//     } else {
//         return Some(optimizers::mcmc::mcmc_naive::MCMCNaive::new(ballot));
//     }
// }

// fn run_trial<O: Optimizer>(trial: usize, rounds: usize, start_seed: u64, optimizer: &mut O) -> (Duration, Vec<Vec<Student>>) {
fn run_trial(trial: usize, rounds: usize, start_seed: u64, optimizer: &mut Box<dyn Optimizer>) -> (Duration, Vec<Vec<Student>>) {
    crate::log_info!(format!("starting trial {} with {} rounds", trial, rounds), "optimizer");
    let time_before_optimize = Instant::now();
    let result = optimizer.optimize(rounds);
    let optimized_time = time_before_optimize.elapsed();
    crate::log_info!(format!("finished trial {} in {} nanos", trial, optimized_time.as_nanos()), "optimizer");
    optimizer.reseed(start_seed + (trial + 1) as u64);
    (optimized_time, result)
}
