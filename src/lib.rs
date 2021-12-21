use std::{env, fs};


#[derive(Debug)]
pub struct BrainfuckFile{
    pub char_vector: Vec::<char>
}
impl BrainfuckFile{
    pub fn read_file(mut args: env::Args) -> BrainfuckFile{
        args.next();
        let file = args.next().unwrap();
        let contents = fs::read_to_string(file).unwrap();
        let contents: String = contents.trim().parse().unwrap();
        let mut char_vec = Vec::new();
        for i in contents.chars(){
            match i {
                '>' => char_vec.push(i),
                '<' => char_vec.push(i),
                '+' => char_vec.push(i),
                '-' => char_vec.push(i),
                '.' => char_vec.push(i),
                ',' => char_vec.push(i),
                '[' => char_vec.push(i),
                ']' => char_vec.push(i),
                _ => continue
            };
        };
        BrainfuckFile{char_vector: char_vec}
    }
}