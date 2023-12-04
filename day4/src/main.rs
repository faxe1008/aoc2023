use std::cmp;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
struct ScratchTicket {
    winners: HashSet<u32>,
    picks: HashSet<u32>,
}

impl ScratchTicket {
    fn from_string(text: &str) -> Self {
        let card_number_part: &str = text.split(':').nth(1).unwrap();
        let number_parts: Vec<&str> = card_number_part.split('|').collect();

        let winning_numbers: HashSet<u32> = HashSet::from_iter(
            number_parts[0]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap()),
        );

        let picked_numbers: HashSet<u32> = HashSet::from_iter(
            number_parts[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap()),
        );

        Self {
            winners: winning_numbers,
            picks: picked_numbers,
        }
    }
}

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let mut tickets: Vec<ScratchTicket> = Vec::new();
    for line in text.split('\n') {
        let ticket = ScratchTicket::from_string(line);
        tickets.push(ticket);
    }

    let ticket_points: Vec<u32> = tickets
        .iter()
        .map(|ticket| {

            let matches : i32 = ticket.winners.intersection(&ticket.picks).count() as i32;
            if matches > 0 {
                2u32.pow(cmp::max(
                    matches - 1,
                    0,
                ) as u32)
            } else {
                0
            }
        })
        .collect();
    let point_sum:u32 = ticket_points.iter().sum();
    println!("Ticket Points: {:?}", ticket_points);
    println!("Sum: {:?}", point_sum);
}

fn riddle_part_two(file_path: &String) {}

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
