use std::env;

fn main() {
    let structboy = rustfuck::BrainfuckFile::read_file(env::args());
    rustfuck::BrainfuckFile::interpert_bf(&structboy.char_vector);
}
