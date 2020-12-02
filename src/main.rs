use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut data = File::open("../day1.txt").unwrap();
    let mut contents = String::new();
    data.read_to_string(&mut contents).unwrap();
    let numbers: Vec<usize> = contents.lines().map(|line| line.parse::<usize>().unwrap()).collect();
    parse_input_part_one(&numbers);
    parse_input_part_two(&numbers);

    Ok(())
}

fn parse_input_part_one(numbers: &Vec<usize>) {
    let mut matching_number: usize = 0;
    let mut i = 0usize;
    let mut z = 1usize;
    while matching_number == 0 {
        let l = numbers[i];
        let r = numbers[z];
        if l + r == 2020 {
            println!("l: {}, r: {}", l, r);
            matching_number = l * r;
        } else if z == numbers.len() - 1 {
            i = 1 + i;
            z = 2 + i;
        } else {
            z = z + 1;
        }
    }

    println!("Matching Number {}", matching_number);

    // Ok(contents);
}

fn parse_input_part_two(numbers: &Vec<usize>) {
    let mut matching_number: usize = 0;
    let mut i = 0usize;
    let mut z = 1usize;
    let mut x = 2usize;
    let length = numbers.len();
    while matching_number == 0 {
        let l = numbers[i];
        let r = numbers[z];
        let fx = numbers[x];
        if l + r + fx == 2020 {
            matching_number = l * r * fx;
            break;
        } else if z == length - 2 {
            i = i + 1;
            z = i + 1;
            x = i + 2;
        } else if x == length - 1 {
            z = z + 1;
            x = z + 1;
        } else {
            x = x + 1;
        }
    }
    println!("MATCHING_NUMBER: {} of {}", matching_number, length);

}