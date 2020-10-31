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
    data_file.write("\nalgo:".as_ref()).expect("unable to write to file");

    for trial in trials {
        let start_seed: u64 = 0;
        let trials: usize = 20;
        let rounds: usize = 10000;
        let threads: usize = 10;
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
                    let (optimized_time, result) = run_trial(trial_num, rounds, start_seed, &mut optimizer, t);
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

fn select_optimizer(trial_name: &str, ballot: &ballot::Ballot) -> Box<dyn Optimizer> {
    match trial_name {
        "swap-naive" => { Box::new(optimizers::swap_naive::SwapNaive::new(ballot, 0)) }
        "multi" => { Box::new(optimizers::multi_dist::MultiDist::new(ballot, 0)) }
        "minimax-friends" => { Box::new(optimizers::mcmc::minimax_friends::MinimaxFriends::new(ballot)) }
        "minimax" => { Box::new(optimizers::mcmc::minimax::Minimax::new(ballot)) }
        "deans" => { Box::new(optimizers::deans_algorithm::DeansAlgorithm::new(ballot)) }
        "network" => { Box::new(optimizers::network::NetworkOptimizer::new(ballot, 10.0, 10.0)) }
        "swap" => { Box::new(optimizers::mcmcswap::mcmc_swap::MCMCSWAP::new(ballot)) }
        "gibb" => { Box::new(optimizers::mcmcswap::mcmc_gibbs::MCMCGibbs::new(ballot)) }
        _ => { Box::new(optimizers::mcmc::mcmc_naive::MCMCNaive::new(ballot)) }
    }
}

fn run_trial(trial: usize, rounds: usize, start_seed: u64, optimizer: &mut Box<dyn Optimizer>, thread_num: usize) -> (Duration, Vec<Vec<Student>>) {
    crate::log_info!(format!("starting trial {} with {} rounds", trial, rounds), format!("optimizer-{}", thread_num));
    let time_before_optimize = Instant::now();
    let result = optimizer.optimize(rounds);
    let optimized_time = time_before_optimize.elapsed();
    crate::log_info!(format!("finished trial {} in {} nanos", trial, optimized_time.as_nanos()), format!("optimizer-{}", thread_num));
    optimizer.reseed(start_seed + (trial + 1) as u64);
    (optimized_time, result)
}
