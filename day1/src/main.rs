use std::env;
use std::fs;
use std::ops::Index;

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
   
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let lines : Vec<&str> = text.split('\n').collect();
    let mut numbers : Vec<u32> = Vec::new();

    
    let number_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    #[derive(PartialEq)]
    enum SearchDir {
        Left,
        Right
    }

    let get_first_number = |s: &str, dir: SearchDir| {

        let mut first_index = match dir {
            SearchDir::Left => s.len(),
            SearchDir::Right => 0
        };
        let mut digit_value:u32 = 0;

        // Search for word text
        for (word_index, number_word) in number_words.iter().enumerate() {
            let search_res = match dir {
                SearchDir::Left => s.find(number_word),
                SearchDir::Right => s.rfind(number_word)
            };
            if let Some(index) = search_res {

                match dir {
                    SearchDir::Left => {
                        if index <= first_index {
                            first_index = index;
                            digit_value = word_index as u32 + 1;
                        }
                    },
                    SearchDir::Right => {
                        if index >= first_index {
                            first_index = index;
                            digit_value = word_index as u32 + 1;
                        }
                    }
                };
            }
        }

        // Search for the digits
        let search_res = match dir {
            SearchDir::Left => s.find(|x:char| x.is_digit(10)),
            SearchDir::Right =>s.rfind(|x: char| x.is_digit(10))
        };
        if let Some(index) = search_res {
            match dir {
                SearchDir::Left => {
                    if index <= first_index {
                        first_index = index;
                        digit_value = s.chars().nth(index).unwrap().to_digit(10).unwrap();
                    }
                },
                SearchDir::Right => {
                    if index >= first_index {
                        first_index = index;
                        digit_value = s.chars().nth(index).unwrap().to_digit(10).unwrap();
                    }
                }
            }
        }

        digit_value
    };

    for line in lines {
        let left = get_first_number(line, SearchDir::Left);
        let right = get_first_number(line, SearchDir::Right);

        let number = left * 10 + right;
        println!("{:?} => {:?}", line, number);
        numbers.push(number);
    }
    let sum : u32= numbers.into_iter().sum();
    println!("{:?}", sum);

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