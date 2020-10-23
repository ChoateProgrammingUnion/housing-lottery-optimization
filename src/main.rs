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

    let mut optimizer = optimizers::multi_dist::MultiDist::new(&ballot, 0);
    // let mut optimizer = optimizers::mcmc::mcmc_swap::MCMCSWAP::new(&ballot);
    //let mut optimizer = optimizers::mcmc::minimax::Minimax::new(&ballot);
    //let mut optimizer = optimizers::mcmc::mcmc_naive::MCMCNaive::new(&ballot);
    //let mut optimizer = optimizers::deans_algorithm::DeansAlgorithm::new(&ballot);

    let start_seed: u64 = 1000;
    let trials: usize = 10;
    let rounds: usize = 10000;
    let mut results: Vec<Vec<Vec<Student>>> = vec![];
    let mut times: Vec<Duration> = vec![];
    let write_first_allocation_vector = false;

    for trial in 0..trials {
        crate::log_info!(format!("starting trial {} with {} rounds", trial, rounds), "optimizer");
        let time_before_optimize = Instant::now();
        let result = optimizer.optimize(rounds);
        let optimized_time = time_before_optimize.elapsed();
        results.push(result);
        times.push(optimized_time);
        crate::log_info!(format!("finished trial {} in {} nanos", trial, optimized_time.as_nanos()), "optimizer");
        optimizer.reseed(start_seed + (trial + 1) as u64);
    }

    crate::log_info!("writing", "output");
    if write_first_allocation_vector { output::write_output(&results[0], &ballot); }
    data_output::write_output(&results, &ballot, &times);
    crate::log_info!("finished", "output");
}
