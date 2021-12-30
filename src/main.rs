use std::env;

fn main() {
    rustfuck::interpert_bf(rustfuck::read_file(env::args()));
}

