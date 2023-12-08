use core::panic;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::env;
use std::fs;


struct WasteLandMap {
    left_right_list:  Vec<usize>,
    map: HashMap<String, [String; 2]>
}

impl WasteLandMap {
    fn from_str(text: &str) -> Self {
        let lines : Vec<&str> = text.split('\n').collect();

        let left_right_list : Vec<usize> = lines[0].chars().map(|c: char| if c == 'L' { 0 } else { 1}).collect();

        let mut map: HashMap<String, [String; 2]> = HashMap::new();

        for line in lines.iter().skip(2){
            let parts : Vec<&str> = line.split('=').collect();

            let cur_pos = parts[0].trim();
            let left_next = &parts[1].split(',').nth(0).unwrap().trim()[1..];
            
            let r_tmp = &parts[1].split(',').nth(1).unwrap().trim();
            let right_next = &r_tmp[0..r_tmp.len()-1]; 
    
    
            map.insert(cur_pos.to_string(), [left_next.to_string(), right_next.to_string()]);
        }
        Self {
            left_right_list,
            map
        }
    }
}


fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");


    let map = WasteLandMap::from_str(&text);

    let mut cur_pos = "AAA";
    let mut step_count = 0;

    let mut cur_step_in_instructions = 0;

    loop {
        if cur_pos == "ZZZ" {
            break;
        }

        let path_index = map.left_right_list[cur_step_in_instructions];
        cur_step_in_instructions = (cur_step_in_instructions + 1) % map.left_right_list.len();

        println!("Taking path: {:?}", map.map[cur_pos][path_index]); 
        cur_pos = &map.map[cur_pos][path_index];

        step_count += 1;
    }
    println!("Steps: {}", step_count);

}

fn gcd(mut a: usize, mut b: usize) ->  usize{
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let map = WasteLandMap::from_str(&text);

   
    let mut cur_positions : Vec<&String> = map.map.keys().filter(|key| key.ends_with('A')).collect();
    println!("Found {} starting positions: {:?}", cur_positions.len(), cur_positions);


    let mut terminations : Vec<usize> = Vec::new();
    for i in 0..cur_positions.len() {

        let mut step_count = 0;
        let mut cur_step_in_instructions = 0;

        loop {

            if cur_positions[i].ends_with('Z') {
                break;
            }
    
            let path_index = map.left_right_list[cur_step_in_instructions];
            cur_step_in_instructions = (cur_step_in_instructions + 1) % map.left_right_list.len();
            cur_positions[i] = &map.map[cur_positions[i]][path_index];
            step_count += 1;
        }
        println!("{} terminates after: {}", cur_positions[i], step_count);
        terminations.push(step_count);
    }

    let mut lcm = 1;
    for i in &terminations {
        lcm = lcm * (*i as f64 / gcd(lcm, *i) as f64 ).floor() as usize;   
    }
    dbg!(lcm);

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


