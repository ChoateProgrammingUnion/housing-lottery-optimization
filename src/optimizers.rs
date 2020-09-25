pub mod identity;
pub mod mcmc;

use ballot::Ballot;
use ballot::Student;

pub trait Optimizer {
    fn optimize(&self) -> Vec<Vec<Student>>;
    fn objective(&self) -> f64; // the objective function
}
