use core::panic;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::cmp::PartialOrd;
use std::hash::RandomState;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
enum Card {
    Ace=12,
    King=11,
    Queen=10,
    Jet=9,
    Ten=8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five= 3,
    Four= 2,
    Three =1,
    Two =0
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jet,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Unknown Card")
        }
    }
}

#[derive(Debug)]
struct CardHand {
    cards: HashMap<Card, usize>,
    raw_txt: String,
    bid: usize
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CardHandType {
    FiveOfKind = 6,
    FourOfKind= 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0
}

impl CardHand {

    fn find_by_count(&self, count: usize) -> Option<Card> {
        self.cards.iter().find_map(|(&card, &c)| 
            if c == count {
                Some(card)
            }else {
                None
            }
        )
    }

    fn get_type(&self) -> CardHandType {
       
       if self.find_by_count(5).is_some() {
        return CardHandType::FiveOfKind;
       } else if self.find_by_count(4).is_some() {
         return CardHandType::FourOfKind;
       }

       if self.find_by_count(3).is_some(){
            if self.cards.len() == 2 {
                return CardHandType::FullHouse;
            } else if self.cards.len() == 3 {
                return CardHandType::ThreeOfKind;
            } else {
                panic!("WTF");
            }
       }

       if let Some(card) = self.find_by_count(2){
            if self.cards.iter().find_map(|(cd, &count)| if count == 2 && *cd != card { Some(cd)} else { None }).is_some() {
                return CardHandType::TwoPair;
            }else {
                return CardHandType::OnePair;
            }
       }

       CardHandType::HighCard
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.raw_txt == other.raw_txt
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = self.get_type().cmp(&other.get_type());
        if res != Ordering::Equal {
            return Some(res);
        }

        for (self_char, other_char) in self.raw_txt.chars().zip(other.raw_txt.chars()) {
                
                let self_card = Card::from(self_char);
                let other_card = Card::from(other_char);

                match self_card.cmp(&other_card) {
                    Ordering::Equal => continue,
                    e => return Some(e),
                };
        }

        return None;
    }
}

impl Eq for CardHand {

}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let res = self.get_type().cmp(&other.get_type());
        if res != Ordering::Equal {
            return res;
        }

        for (self_char, other_char) in self.raw_txt.chars().zip(other.raw_txt.chars()) {
                
                let self_card = Card::from(self_char);
                let other_card = Card::from(other_char);

                match self_card.cmp(&other_card) {
                    Ordering::Equal => continue,
                    e => return e,
                };
        }

        return Ordering::Equal;
    }
}

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let lines : Vec<&str> = text.split('\n').collect();

    let mut hands : Vec<CardHand> = lines.iter().filter(|l| !l.is_empty()).map(|line| 
        
        CardHand {
            cards: line.split(' ').nth(0).unwrap().chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(Card::from(c)).or_insert(0) += 1;
                map
            }),
            raw_txt: line.split(' ').nth(0).unwrap().to_string(),
            bid: line.split(' ').nth(1).unwrap().parse::<usize>().unwrap()
        }
    ).collect();

    
    hands.sort();
   
    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (rank + 1);
    }
    println!("{:?}", winnings);

    

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
