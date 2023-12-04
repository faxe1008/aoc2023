use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::collections::VecDeque;

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

    fn points(&self) -> u32 {
        let matches = self.winners.intersection(&self.picks).count() as i32;
        if matches > 0 {
            2u32.pow(cmp::max(matches - 1, 0) as u32)
        } else {
            0
        }
    }

    fn matching_numbers(&self) -> u32 {
        self.winners.intersection(&self.picks).count() as u32
    }
 }

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let mut tickets: Vec<ScratchTicket> = Vec::new();
    for line in text.split('\n') {
        let ticket = ScratchTicket::from_string(line);
        tickets.push(ticket);
    }

    let ticket_points: Vec<u32> = tickets.iter().map(|ticket| ticket.points()).collect();
    let point_sum: u32 = ticket_points.iter().sum();
    println!("Ticket Points: {:?}", ticket_points);
    println!("Sum: {:?}", point_sum);
}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let mut tickets: Vec<ScratchTicket> = Vec::new();
    for line in text.split('\n') {
        let ticket = ScratchTicket::from_string(line);
        tickets.push(ticket);
    }


    let mut ticket_pile: HashMap<usize, usize> = HashMap::new();
    let mut ticket_queue : VecDeque<(usize, &ScratchTicket)> = VecDeque::new();

    let ticket_count = tickets.len();

    let mut incr_ticket_count = |ticket_index| {
        ticket_pile.insert(ticket_index, ticket_pile.get(&ticket_index).unwrap_or(&0) + 1);
    };

    for (ticket_index, ticket) in tickets.iter().enumerate() {
        let ticket_points = ticket.matching_numbers();
        incr_ticket_count(ticket_index);

        let end_index = cmp::min(ticket_index + 1 + ticket_points as usize, ticket_count);
        for copy_index in ticket_index+1..end_index {
            let copy_ticket = &tickets[copy_index];

            ticket_queue.push_back((copy_index, copy_ticket));
        }
    }

    while let Some((ticket_index, ticket)) = ticket_queue.pop_front() {
        let ticket_points = ticket.matching_numbers();
        incr_ticket_count(ticket_index);

        let end_index = cmp::min(ticket_index + 1 + ticket_points as usize, ticket_count);
        for copy_index in ticket_index+1..end_index {
            let copy_ticket = &tickets[copy_index];

            ticket_queue.push_back((copy_index, copy_ticket));
        }
    }

    let ticket_count : usize = ticket_pile.iter().map(|(k,v)| v).sum();
    println!("Ticket Count: {:?}", ticket_count);
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
