use core::panic;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::env;
use std::fs;


fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");


    let lines : Vec<&str> = text.split('\n').collect();


    let left_right_list : Vec<usize> = lines[0].chars().map(|c: char| if c == 'L' { 0 } else { 1}).collect();


    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();
    for line in lines.iter().skip(2) {
        // AAA = (BBB, CCC)

        let parts : Vec<&str> = line.split('=').collect();

        let cur_pos = parts[0].trim();
        let left_next = &parts[1].split(',').nth(0).unwrap().trim()[1..];
        
        let r_tmp = &parts[1].split(',').nth(1).unwrap().trim();
        let right_next = &r_tmp[0..r_tmp.len()-1]; 


        map.insert(cur_pos, [left_next, right_next]);
    }


    let mut cur_pos = "AAA";
    let mut step_count = 0;

    let mut cur_step_in_instructions = 0;

    loop {
        if cur_pos == "ZZZ" {
            break;
        }

        let path_index = left_right_list[cur_step_in_instructions];
        cur_step_in_instructions = (cur_step_in_instructions + 1) % left_right_list.len();

        println!("Taking path: {:?}", map[cur_pos][path_index]); 
        cur_pos = map[cur_pos][path_index];

        step_count += 1;
    }
    println!("Steps: {}", step_count);

}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

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
        .expect("Error parsing ridle num");

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


