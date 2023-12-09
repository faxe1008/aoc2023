use core::panic;
use std::env;
use std::fs;


fn get_time_rows(text: &str) -> Vec<Vec<isize>> {
    text.split('\n').filter(|x| !x.is_empty()).map(|line|
        line.trim().split(' ').map(|t| t.parse::<isize>().unwrap()).collect()
    ).collect()
}

fn get_prediction_for_time_row(time_row: &Vec<isize>) -> isize {


    let mut time_row_diffs : Vec<Vec<isize>> = vec![time_row.clone()];


    loop {

        let cur_pos = time_row_diffs.last().unwrap();

        if cur_pos.iter().all(|&y| y == 0) {
            break;
        }

        let mut acc_vec : Vec<isize> = Vec::new();
        for i in 0..cur_pos.len()-1 {
            acc_vec.push(cur_pos[i+1] - cur_pos[i]);
        }
        time_row_diffs.push(acc_vec);
    }


    
    time_row_diffs.iter().map(|x| x.last().unwrap()).sum()
}



fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let time_rows  = get_time_rows(&text);


    let sum : isize = time_rows.iter().map(|row| get_prediction_for_time_row(&row)).sum();
    dbg!(sum);
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


