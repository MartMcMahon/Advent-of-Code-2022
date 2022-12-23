use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = io::BufReader::new(File::open("input_1.txt").unwrap()).lines();

    let mut calorie_count: Vec<i32> = Vec::new();
    let mut elf = 0;
    for line in lines {
        if line.as_ref().unwrap() != "" {
            elf += line.unwrap().parse::<i32>().unwrap();
        } else {
            // end of elf
            calorie_count.push(elf);
            elf = 0;
        }
    }
    println!(
        "highest calorie_count: {}",
        calorie_count.iter().max().unwrap()
    );
}
