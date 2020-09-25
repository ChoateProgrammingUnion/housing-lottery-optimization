use optimizers::Optimizer;

mod ballot;
mod input;
mod optimizers;
// mod optimizer;

// move to ballot.rs once implemented
fn normalize(student: ballot::Student) -> ballot::Student {
    unimplemented!();
}

fn scale(student: ballot::Student) -> ballot::Student {
    unimplemented!();
}

fn identity(student: ballot::Student) -> ballot::Student {
    return student;
}

fn main() {
    let ballot = input::load_input(identity);
    let identity = optimizers::identity::Identity::new(ballot);
    let result = identity.optimize();
    println!("{:?}", result);
}
