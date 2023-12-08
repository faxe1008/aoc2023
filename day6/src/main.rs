use core::panic;
use std::cmp;
use std::env;
use std::fs;
use std::result;


fn does_win_race(hold_duration: f64, race_duration: f64, race_distance: f64) -> bool {
    return hold_duration * race_duration -  hold_duration * hold_duration >= race_distance
}

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");
    let lines: Vec<&str> = text.split('\n').collect();

    let race_durations : Vec<f64> = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    let race_distances : Vec<f64> = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    /*
        speed = hold_duration
        duration_for_movement = race_duration - hold_duration

        distance_traveled = hold_duration * (race_duration - hold_duration)
        distance_traveled = hold_duration * race_duration - hold_duration²

        #######################
        distance_traveled = hold_duration * (race_duration - 1 - hold_duration)
        distance_traveled = hold_duration * (race_duration-1) - hold_duration²


        #######################


        0 = - hold_duration² + race_duration * hold_duration - distance_traveled

        so, solve:
        0 = -1 * x² + t * x - d
     */

    let mut hold_duration_rng_lens = Vec::<f64>::new();
    for (t, d) in race_durations.iter().zip(race_distances) {
        
        let d = d + 1.0;

        let x1 = (-t + (t * t - (4.0 * -1.0 * -d)).sqrt()) / (2.0*-1.0);
        let x2 = (-t - (t * t - (4.0 * -1.0 * -d)).sqrt()) / (2.0*-1.0);

        let lower = x1.ceil();
        let upper = x2.floor();

        println!("t: {}, d: {} == lower: {}, upper: {} ======= {}, {}", t + 1.0, d, lower, upper, x1, x2);
        let rng = upper - lower + 1.0;

        hold_duration_rng_lens.push(rng);
    }
    dbg!(hold_duration_rng_lens.iter().product::<f64>());
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
