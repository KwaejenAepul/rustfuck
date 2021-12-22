use std::{env, fs};

#[derive(Debug)]
pub struct BrainfuckFile {
    pub char_vector: Vec<char>,
}
impl BrainfuckFile {
    pub fn read_file(mut args: env::Args) -> BrainfuckFile {
        args.next();
        let file = args.next().unwrap();
        let contents = fs::read_to_string(file).unwrap();
        let contents: String = contents.trim().parse().unwrap();
        let mut char_vec = Vec::new();
        for i in contents.chars() {
            match i {
                '>' => char_vec.push(i),
                '<' => char_vec.push(i),
                '+' => char_vec.push(i),
                '-' => char_vec.push(i),
                '.' => char_vec.push(i),
                ',' => char_vec.push(i),
                '[' => char_vec.push(i),
                ']' => char_vec.push(i),
                _ => continue,
            };
        }
        BrainfuckFile {
            char_vector: char_vec,
        }
    }
    pub fn interpert_bf(char_vec: &Vec<char>) {
        let mut i: usize = 0;
        let mut cell_array = [0; 30000];
        let mut cell_pointer: usize = 0;
        let mut squarebracket_index: usize = 0;
        while i < char_vec.len() {
            match char_vec[i] {
                '>' => cell_pointer = cell_pointer + 1,
                '<' => cell_pointer = cell_pointer - 1,
                '+' => cell_array[cell_pointer] = cell_array[cell_pointer] + 1,
                '-' => cell_array[cell_pointer] = cell_array[cell_pointer] - 1,
                '.' => continue,
                ',' => continue,
                '[' => squarebracket_index = i,
                ']' => {
                    if cell_array[cell_pointer] != 0 {
                        i = squarebracket_index
                    }
                }
                _ => continue,
            };
        }
    }
}
