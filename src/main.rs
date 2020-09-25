use optimizers::Optimizer;

mod ballot;
mod input;
mod optimizers;
// mod optimizer;

fn main() {
    let ballot = input::load_input();
    let identity = optimizers::identity::Identity::new(ballot);
    let result = identity.optimize();
    println!("{:?}", result);
}
