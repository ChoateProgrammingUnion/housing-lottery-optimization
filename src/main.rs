mod ballot;
mod input;
mod output;
mod data_output;
mod optimizers;
mod logger;

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

    // let optimizer = optimizers::multi_dist::MultiDist::new(&ballot, 0);
    // let optimizer = optimizers::mcmcswap::mcmc_swap::MCMCSWAP::new(&ballot);
    // let optimizer = optimizers::mcmc::minimax::Minimax::new(&ballot);
    // let optimizer = optimizers::mcmc::mcmc_naive::MCMCNaive::new(&ballot);
    // let optimizer = optimizers::deans_algorithm::DeansAlgorithm::new(&ballot);
    // let optimizer = optimizers::network::NetworkOptimizer::new(&ballot, 10.0); // use with normalize or scale; expects 0-1 range
    //let optimizer = optimizers::mcmc::minimax::Minimax::new(&ballot);
    let optimizer = optimizers::mcmcswap::mcmc_gibbs::MCMCGibbs::new(&ballot);

    let start_seed: u64 = 0;
    let trials: usize = 100;
    let rounds: usize = 10000;
    let threads: usize = 8;
    let mut results: Vec<Vec<Vec<Student>>> = vec![];
    let mut times: Vec<Duration> = vec![];
    let write_first_allocation_vector = false;

    let mut handles: Vec<std::thread::JoinHandle<(Vec<Duration>, Vec<Vec<Vec<Student>>>)>> = vec![];
    for t in 0..threads {
        let new_optimizer = optimizer.clone();
        handles.push(std::thread::spawn(move || {
            let mut optimizer = new_optimizer;
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
    data_output::write_output(&results, &ballot, &times);
    crate::log_info!("finished", "output");
}

fn run_trial<O: Optimizer>(trial: usize, rounds: usize, start_seed: u64, optimizer: &mut O) -> (Duration, Vec<Vec<Student>>) {
    crate::log_info!(format!("starting trial {} with {} rounds", trial, rounds), "optimizer");
    let time_before_optimize = Instant::now();
    let result = optimizer.optimize(rounds);
    let optimized_time = time_before_optimize.elapsed();
    crate::log_info!(format!("finished trial {} in {} nanos", trial, optimized_time.as_nanos()), "optimizer");
    optimizer.reseed(start_seed + (trial + 1) as u64);
    (optimized_time, result)
}
