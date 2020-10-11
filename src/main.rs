mod ballot;
mod input;
mod output;
mod data_output;
mod optimizers;
#[macro_use] mod logger;

use optimizers::Optimizer;

// Timing
use std::time::Instant;

fn scale(student: ballot::Student) -> ballot::Student {
    unimplemented!();
}

fn identity(student: ballot::Student) -> ballot::Student {
    return student;
}

fn main() {
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
    output::write_output(&result, &ballot);
    data_output::write_output(&result, &ballot, &optimized_time);
    crate::log_info!("finished", "output");
}
