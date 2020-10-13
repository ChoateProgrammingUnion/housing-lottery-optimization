mod ballot;
mod input;
mod output;
mod data_output;
mod optimizers;
mod logger;

use optimizers::Optimizer;

use std::time::Instant;

extern crate log;
use log::LevelFilter;


fn main() {
    // Change this to set the log level
    // LevelFilter::Off   - No logging (USE THIS FOR BENCHMARKS AS LOGS TAKE TIME TO PRINT)
    // LevelFilter::Error - Print errors (nonfatal errors that are logged)
    // LevelFilter::Info  - Print info messages (and errors)
    // LevelFilter::Debug - Print debug messages (and info, error)
    // LevelFilter::Trace - Print trace messages (and info, error, debug) (a lot of messages)
    logger::init(LevelFilter::Off);

    crate::log_info!("processing...", "input");
    let ballot = input::load_input(ballot::identity);
    crate::log_info!("successfully processed", "input");

    let mut identity = optimizers::multi_dist::MultiDist::new(&ballot, 0, 10.0);
    //let mut identity = optimizers::mcmc::mcmc_naive::MCMCNaive::new(&ballot);
    //let mut identity = optimizers::deans_algorithm::DeansAlgorithm::new(&ballot);

    // println!("How many rounds?");
    // let mut rounds_input = String::new();
    // io::stdin()
    //     .read_line(&mut rounds_input)
    //     .expect("Not a valid input!");
    // let rounds = rounds_input.trim().parse::<usize>().expect("Not a usize");
    let rounds: usize = 100000;

    crate::log_info!("starting", "optimizer");
    let time_before_optimize = Instant::now();
    let result = identity.optimize(rounds);
    let optimized_time = time_before_optimize.elapsed();
    crate::log_info!("finished", "optimizer");

    crate::log_info!("writing", "output");
    output::write_output(&result, &ballot);
    data_output::write_output(&result, &ballot, &optimized_time);
    crate::log_info!("finished", "output");
}
