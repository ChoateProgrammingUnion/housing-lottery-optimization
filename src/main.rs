mod ballot;
mod input;
mod output;
mod data_output;
mod optimizers;
mod logger;
mod network;

use optimizers::Optimizer;
use network::NetworkOptimizer;

use std::time::Instant;
use std::process::Command;

extern crate log;
extern crate petgraph;
use log::LevelFilter;
//use cpython::{Python, PyDict, PyResult};

fn main() {

    // Change this to set the log level
    // LevelFilter::Off   - No logging (USE THIS FOR BENCHMARKS AS LOGS TAKE TIME TO PRINT)
    // LevelFilter::Error - Print errors (nonfatal errors that are logged)
    // LevelFilter::Info  - Print info messages (and errors)
    // LevelFilter::Debug - Print debug messages (and info, error)
    // LevelFilter::Trace - Print trace messages (and info, error, debug) (a lot of messages)
    logger::init(LevelFilter::Off);

    crate::log_info!("processing...", "input");
    Command::new("python3")
            .arg("generate_ballots.py")
            .output()
            .expect("failed to execute process");

        

    //let ballot = input::load_input(ballot::identity);
    let ballot = input::load_input(ballot::normalize);
    crate::log_info!("successfully processed", "input");

    //let mut identity = optimizers::multi_dist::MultiDist::new(&ballot, 0, 10.0);
    let mut identity = optimizers::mcmc::mcmc_swap::MCMCSWAP::new(&ballot);
    //let mut identity = optimizers::mcmc::minimax::Minimax::new(&ballot);
    //let mut identity = optimizers::mcmc::mcmc_naive::MCMCNaive::new(&ballot);
    //let mut identity = optimizers::deans_algorithm::DeansAlgorithm::new(&ballot);


    // println!("How many rounds?");
    // let mut rounds_input = String::new();
    // io::stdin()
    //     .read_line(&mut rounds_input)
    //     .expect("Not a valid input!");
    // let rounds = rounds_input.trim().parse::<usize>().expect("Not a usize");
    
    for x in 0..4 {
        let num: usize = 10;
        let mut rounds: usize = 10*num.pow(x as u32);

        crate::log_info!("starting", "optimizer");
        let time_before_optimize = Instant::now();
        let result = identity.optimize(rounds);
        let optimized_time = time_before_optimize.elapsed();
        crate::log_info!("finished", "optimizer");

        crate::log_info!("writing", "output");
        output::write_output(&result, &ballot);
        data_output::write_output(&result, &ballot, &optimized_time, &(x+1).to_string());
        crate::log_info!("finished", "output");
        
    }
    Command::new("python3")
            .arg("graph.py")
            .output()
            .expect("failed to execute process");
}
