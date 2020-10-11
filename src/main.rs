use optimizers::Optimizer;

mod ballot;
mod input;
use std::io;
mod output;
mod optimizers;


fn main() {
    let ballot = input::load_input(ballot::identity);
    let mut identity = optimizers::identity::Identity::new(&ballot);

    println!("How many rounds?");
    let mut rounds_input = String::new();
    io::stdin()
        .read_line(&mut rounds_input)
        .expect("Not a valid input!");
    let rounds = rounds_input.trim().parse::<usize>().expect("Not a usize");

    let result = identity.optimize(rounds);
    println!("{:?}", result);
  
    output::write_output(&result, &ballot);
}
