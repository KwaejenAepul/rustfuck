use std::{env, fs};
/*
needs:  - input
        - output
        - print
        - loop
*/

pub fn read_file(mut args: env::Args) -> String {
    args.next();
    let file = args.next().unwrap();
    let contents = fs::read_to_string(file).unwrap();
    contents
}
pub fn interpert_bf(bf: String) {
    let mut data_cell_array = [0u32; 30000];
    let mut cursor: usize = 0;
    let mut i = 0;
    let mut loops_vec: Vec<u8> = Vec::new();
    while i < bf.len() {
        println!("{}{}", bf.chars().nth(i).unwrap(), i);
        match bf.chars().nth(i).unwrap() {
            '>' => {
                cursor += 1;
                i += 1;
            }
            '<' => {
                cursor -= 1;
                i += 1;
            }
            '+' => {
                data_cell_array[cursor] += 1;
                i += 1;
            }
            '-' => {
                data_cell_array[cursor] += 1;
                i += 1;
            }
            '.' => {
                println!("{:?}", bf.chars().nth(i).unwrap());
                i += 1;
            }
            ',' => {
                i += 1;
            }
            '[' => {
                loops_vec.push(i as u8);
                i += 1;
            }
            ']' => {
                if data_cell_array[cursor] != 0 {
                    if loops_vec.len() == 0 {
                        panic!("\nno start of loop \nerror at character index: {}", i);
                    }
                    i = loops_vec[loops_vec.len() - 1] as usize
                }else {
                    loops_vec.pop();
                    i += 1;
                }
            }
            _ => i += 1,
        };
    }
}
