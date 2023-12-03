use std::env;
use std::fs;

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let lines : Vec<&str> = text.split('\n').collect();
    let mut numbers : Vec<u32> = Vec::new();

    for line in lines {
        let mut chrs = line.chars();

        let first_digit: u32 = chrs.clone().find(|c| c.is_digit(10)).unwrap_or('0').to_digit(10).unwrap();
        let last_digit: u32 = chrs.rfind(|c| c.is_digit(10)).unwrap_or('0').to_digit(10).unwrap();

        numbers.push(first_digit * 10 + last_digit);
    }
    let sum : u32= numbers.into_iter().sum();
    println!("{:?}", sum);
}

fn riddle_part_two(file_path: &String) {
   
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Provide the input text file!");
    }
    let riddle_num: u32 = args
        .get(1)
        .unwrap()
        .parse()
        .expect("Error parsing riddle num");

    match riddle_num {
        1 => {
            riddle_part_one(args.get(2).unwrap());
        }
        2 => {
            riddle_part_two(args.get(2).unwrap());
        }
        _ => {
            panic!("Unknown riddle part number");
        }
    };
}