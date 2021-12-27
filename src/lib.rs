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
        let mut cell_array = [0u8; 30000];
        let mut cell_pointer: usize = 0;
        let mut squarebracket_index_vec: Vec<u32> = vec![];
        while i < char_vec.len() {
            println!("{} {:?}", i, char_vec[i]);
            match char_vec[i] {
                '>' => cell_pointer = cell_pointer + 1,
                '<' => cell_pointer = cell_pointer - 1,
                '+' => cell_array[cell_pointer] = cell_array[cell_pointer] + 1,
                '-' => cell_array[cell_pointer] = cell_array[cell_pointer] - 1,
                '.' => println!("{:?}", cell_array[i].make_ascii_lowercase()),
                ',' => continue,
                '[' => squarebracket_index_vec.push(i as u32),
                ']' => {
                    if cell_array[cell_pointer] != 0 {
                        if squarebracket_index_vec.len() == 0{
                            panic!("\nno start of loop \nerror at character index: {}", i);
                        }
                        i = squarebracket_index_vec[squarebracket_index_vec.len() - 1] as usize
                    }else {
                        squarebracket_index_vec.pop();
                    }
                }
                _ => continue,
            };
            i = i +1;
        }
    }
}
