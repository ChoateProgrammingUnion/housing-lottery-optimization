use ballot::Student;
use optimizers::multi_dist::distribution::AllocatedStudent;

fn scaled_high(n: f64) -> f64 {
    n + 0.001
}

fn ballot_percent(student: &Student, idx: usize) -> f64 {
    student.ballot[idx] / student.ballot_sum
}

fn ballot_percent_allocated(student: &AllocatedStudent, idx: usize) -> f64 {
    student.ballot[idx] / student.ballot_sum
}

pub fn student_pick_weight(student: &AllocatedStudent) -> f64 {
    let dissatisfaction = 1.0 - ballot_percent_allocated(student, student.location.0);
    scaled_high(dissatisfaction)
    // dissatisfaction
}

pub fn student_swap_weight(student: &Student, swap_house: usize, current_house: usize) -> f64 {
    // let mut a = ballot_percent(student, swap_house) * (1.0 - ballot_percent(student, current_house));
    // if a < 0.0 { a = 0.0 }
    // a

    let net = ballot_percent(student, swap_house) - ballot_percent(student, current_house);

    if net < 0.0 {
        0.0001
    } else {
        net + 1.0
    }
}

// How much a student wants to move to each house
pub fn house_move_weights(student: &Student) -> Vec<f64> {
    student
        .ballot
        .iter()
        .map(|weight| weight.powf(20.0))
        .collect::<Vec<f64>>()
}
