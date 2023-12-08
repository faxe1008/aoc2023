use core::panic;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::RandomState;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
enum Card {
    Ace = 12,
    King = 11,
    Queen = 10,
    Ten = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
    Jet = -1,
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
            _ => panic!("Unknown Card"),
        }
    }
}

#[derive(Debug)]
struct CardHand {
    cards: HashMap<Card, usize>,
    raw_txt: String,
    bid: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum CardHandType {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl From<&str> for CardHand {
    fn from(value: &str) -> Self {
        CardHand {
            cards: value
                .split(' ')
                .nth(0)
                .unwrap()
                .chars()
                .fold(HashMap::new(), |mut map, c| {
                    *map.entry(Card::from(c)).or_insert(0) += 1;
                    map
                }),
            raw_txt: value.split(' ').nth(0).unwrap().to_string(),
            bid: value.split(' ').nth(1).unwrap().parse::<usize>().unwrap(),
        }
    }
}

impl CardHand {
    fn find_by_count(&self, count: usize) -> Option<Card> {
        let joker_count = *self.cards.get(&Card::Jet).unwrap_or(&0);

        self.cards.iter().find_map(|(&card, &c)| {
            let occs = if card == Card::Jet {
                c - joker_count
            } else {
                c
            };
            if occs + joker_count == count {
                Some(card)
            } else {
                None
            }
        })
    }

    fn get_type(&self) -> CardHandType {
        let joker_count = *self.cards.get(&Card::Jet).unwrap_or(&0);

        if self.find_by_count(5).is_some() {
            return CardHandType::FiveOfKind;
        } else if self.find_by_count(4).is_some() {
            return CardHandType::FourOfKind;
        }

        if self.find_by_count(3).is_some() {
            if self.cards.len() == 2 || (self.cards.len() == 3 && joker_count == 1) {
                return CardHandType::FullHouse;
            } else if self.cards.len() == 3 {
                return CardHandType::ThreeOfKind;
            } else {
                return CardHandType::ThreeOfKind;
            }
        }

        if let Some(card) = self.find_by_count(2) {

            let second_pair_card = self
            .cards
            .iter()
            .find_map(|(cd, &count)| {
                if count == 2 + joker_count && *cd != card {
                    Some(cd)
                } else {
                    None
                }
            });

            // 644J1
            if second_pair_card.is_some() && self.cards.len() == 3
            {
                return CardHandType::TwoPair;
            } else {
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

impl Eq for CardHand {}

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

    let lines: Vec<&str> = text.split('\n').collect();

    let mut hands: Vec<CardHand> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|line| CardHand::from(*line))
        .collect();

    hands.sort();

    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (rank + 1);
    }
    println!("{:?}", winnings);
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


#[cfg(test)]
mod tests {
    use crate::{CardHandType, CardHand};

    #[test]
    fn check_jet_cases() {
        let samples: Vec<(&'static str, CardHandType)> = vec![
            ("5J984 114", CardHandType::OnePair),
            ("787J8 587", CardHandType::FullHouse),
            ("7JJ77 757", CardHandType::FiveOfKind),
            ("A7T4J 166", CardHandType::OnePair),
            ("Q9A7J 843", CardHandType::OnePair),
            ("8J544 827", CardHandType::ThreeOfKind),
            ("9JQ44 950", CardHandType::ThreeOfKind),
            ("963J5 490", CardHandType::OnePair),
            ("7JJ99 91", CardHandType::FourOfKind),
            ("Q6QQJ 571", CardHandType::FourOfKind),
            ("68J8T 395", CardHandType::ThreeOfKind),
            ("5JJK9 285", CardHandType::ThreeOfKind),
            ("A94AJ 636", CardHandType::ThreeOfKind),
            ("777J8 445", CardHandType::FourOfKind),
            ("JTK7J 7", CardHandType::ThreeOfKind),
            ("96J29 249", CardHandType::ThreeOfKind),
            ("7J44Q 603", CardHandType::ThreeOfKind),
            ("T6JT6 538", CardHandType::FullHouse),
            ("T9AAJ 299", CardHandType::ThreeOfKind),
            ("J5559 357", CardHandType::FourOfKind),
            ("T7AJ7 569", CardHandType::ThreeOfKind),
            ("J7TTK 155", CardHandType::ThreeOfKind),
            ("6QJJQ 500", CardHandType::FourOfKind),
            ("5JJ5J 864", CardHandType::FiveOfKind),
            ("6QJJ6 158", CardHandType::FourOfKind),
            ("T7J77 939", CardHandType::FourOfKind),
            ("7J5T7 337", CardHandType::ThreeOfKind),
            ("JQ478 431", CardHandType::OnePair),
            ("QQ9QJ 977", CardHandType::FourOfKind),
            ("TAJQK 710", CardHandType::OnePair),
            ("J4422 678", CardHandType::FullHouse),
            ("J8JJ6 290", CardHandType::FourOfKind),
            ("644J6 659",CardHandType::FullHouse),
            ("J3333 928", CardHandType::FiveOfKind),
            ("J998A 349",  CardHandType::ThreeOfKind),
        ];

        for (txt, cht) in &samples {
            let c = CardHand::from(*txt);
            assert_eq!(c.get_type(), *cht, "{} should be {:?}", c.raw_txt, *cht);
        }

    }
}


