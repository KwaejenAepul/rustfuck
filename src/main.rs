use std::env;

fn main() {
    let structboy = rustfuck::BrainfuckFile::read_file(env::args());
    println!("{:?}", structboy);
}
