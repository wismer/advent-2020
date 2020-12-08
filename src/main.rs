use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    // let mut data = File::open("../day1.txt").unwrap();
    // let mut contents = String::new();
    // data.read_to_string(&mut contents).unwrap();
    // let numbers: Vec<usize> = contents.lines().map(|line| line.parse::<usize>().unwrap()).collect();
    // parse_input_part_one(&numbers);
    // parse_input_part_two(&numbers);
    // parse_day_five();
    day_six();
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

fn parse_day_five() {
    let mut buffer = String::new();
    let mut file = File::open("./day5.txt").unwrap();
    file.read_to_string(&mut buffer);
    let mut seat_id = 0;
    let mut c = 0;
    let mut seats: Vec<Option<usize>> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        seats.push(None);
    }
    for line in buffer.lines() {
        c += 1;
        let id = parse_seating(line.chars().collect());
        if id > seat_id {
            seat_id = id;
        }

        seats[id] = Some(id);
    }

    println!("HIGHEST: {} from {} seats: {:?}", seat_id, c, seats);
}

fn parse_seating(seating: Vec<char>) -> usize {
    let mut row_max = 127usize;
    let mut row_min = 0usize;
    let mut col_max = 7usize;
    let mut col_min = 0usize;
    let mut id = 0;
    for c in seating {
        match c {
            'F' => {
                // take the lower half
                row_max = (row_max + row_min) / 2;
            },
            'B' => {
                // take the upper half
                row_min = ((row_max + row_min) / 2) + 1;
            },
            'L' => {
                col_max = (col_max + col_min) / 2;
            },
            'R' => {
                col_min = ((col_max + col_min) / 2) + 1;
            },
            _ => panic!("shousdl")
        }
    }

    id = row_max * 8 + col_max;
    id
}

fn day_six() {
    let mut buffer = String::new();
    let mut data = File::open("./day6.txt").unwrap();
    data.read_to_string(&mut buffer);

    let mut iter = buffer.lines();
    let mut new_group = false;
    let mut group = String::new();
    let mut total = 0;
    let mut group_count = 0;
    loop {
        let line = iter.next();

        if line.is_none() {
            total = total + parse_answers(&group, group_count);
            break;
        }

        let answers = line.unwrap();
        if answers.len() == 0 {
            total = total + parse_answers(&group, group_count);
            group = String::new();
            group_count = 0;
            // newline, new group
        } else {
            group_count += 1;
            group.push_str(answers);
            println!("push: {}", group);
        }

    }
    println!("TOTAL: {} FROM: {}", total, group);

}

fn parse_answers(answers: &str, group_count: usize) -> usize {
    let number_of_answers = answers.len();
    let mut answered_questions: HashMap<char, usize> = HashMap::new();
    for q in answers.chars() {
        if answered_questions.contains_key(&q) {
            match answered_questions.get_mut(&q) {
                None => {},
                Some(v) => *v += 1
            }
        } else {
            answered_questions.insert(q, 1);
        }
    }
    let mut total = 0;
    for (_, val) in answered_questions {
        if group_count == val {
            total += 1;
        }
    }
    // println!("{:?}, number: {}", answered_questions.len(), number_of_answers);

    // answered_questions.len()
    total
}