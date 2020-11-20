# housing-optimization
![Rust tests](https://github.com/ChoateProgrammingUnion/housing-optimization/workflows/Rust%20tests/badge.svg)

Automating and optimizing the Choate housing lottery allocation system.

Made by Max Fan, Ethan Chapman, Ian Haile, Aiyu Kamate, and Jeffrey Zhou for our simulations and optimizations directed study at Choate Rosemary Hall.

## Motivation
Traditionally, housing allocation at Choate Rosemary Hall was done by hand.
Decades ago, this process was done through a lottery, with students randomly selecting numbers out of a hat.
The students would be allocated to the house of their choosing in the ordering of the numbers they received.
This system was later amended to allow for more flexibility, giving Choate students the system, they have today -- a "random" lottery that takes into account everyone's preferences and priorities.

The optimization techniques available have drastically improved since the 20th century.
It is now feasible for normal laptops to churn through hundreds of thousands of possibilities per second.
In addition to the computational leaps made within the past century, there has been great development in the theory of optimization and resource allocation.
We aim to examine and develop several optimization techniques that can take advantage of these recent developments.

## Usage
The input files are `input.yaml` and `config.yaml`.
Running `cargo run` will generate an `output.yaml` and `data_output.yaml` file.

Build:
```rs
cargo build --release
```
Run:
```rs
cargo run
```

