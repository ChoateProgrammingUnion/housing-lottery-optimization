mod ballot;
mod input;
mod output;
mod data_output;
mod optimizers;
mod logger;

use optimizers::Optimizer;

use std::time::Instant;
use std::io;


fn main() {
    let ballot = input::load_input(ballot::identity);
    logger::init();

    crate::log_info!("processing...", "input");
    let ballot = input::load_input(identity);
    crate::log_info!("successfully processed", "input");

    let mut identity = optimizers::identity::Identity::new(&ballot);

    crate::log_info!("starting", "optimizer");
    let time_before_optimize = Instant::now();
    let result = identity.optimize();
    let optimized_time = time_before_optimize.elapsed();
    crate::log_info!("finished", "optimizer");

    crate::log_info!("writing", "output");

    // println!("How many rounds?");
    // let mut rounds_input = String::new();
    // io::stdin()
    //     .read_line(&mut rounds_input)
    //     .expect("Not a valid input!");
    // let rounds = rounds_input.trim().parse::<usize>().expect("Not a usize");

    let result = identity.optimize(rounds);
    println!("{:?}", result);

    output::write_output(&result, &ballot);
    data_output::write_output(&result, &ballot, &optimized_time);
    crate::log_info!("finished", "output");
}
