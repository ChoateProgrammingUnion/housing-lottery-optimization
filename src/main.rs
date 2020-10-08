use optimizers::Optimizer;

mod ballot;
mod input;
mod output;
mod optimizers;

fn scale(student: ballot::Student) -> ballot::Student {
    unimplemented!();
}

fn identity(student: ballot::Student) -> ballot::Student {
    return student;
}

fn main() {
    let ballot = input::load_input(identity);
    let mut identity = optimizers::identity::Identity::new(&ballot);
    let result = identity.optimize();
    output::write_output(&result, &ballot);
}
