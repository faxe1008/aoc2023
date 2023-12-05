use core::panic;
use std::env;
use std::fs;
use std::ops::Range;

#[derive(Debug, PartialEq)]
enum InformationType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl InformationType {
    fn from_string(text: &str) -> Self {
        if text == "seed" {
            InformationType::Seed
        } else if text == "soil" {
            InformationType::Soil
        } else if text == "fertilizer" {
            InformationType::Fertilizer
        } else if text == "water" {
            InformationType::Water
        } else if text == "light" {
            InformationType::Light
        } else if text == "temperature" {
            InformationType::Temperature
        } else if text == "humidity" {
            InformationType::Humidity
        } else if text == "location" {
            InformationType::Location
        } else {
            dbg!(text);
            panic!("Unknown type");
        }
    }
}

#[derive(Debug, PartialEq)]
struct MappingRule {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

impl MappingRule {
    fn from_string(text: &str) -> Self {
        // 0 15 37
        let numbers: Vec<usize> = text
            .trim()
            .split(' ')
            .filter(|t| !t.is_empty())
            .map(|t| t.parse::<usize>().expect("Error parsing number"))
            .collect();
        if numbers.len() != 3 {
            panic!("Nonsense rule found");
        }

        Self {
            destination_range: Range { start: numbers[0], end: numbers[0] + numbers[2] },
            source_range: Range { start: numbers[1], end: numbers[1] + numbers[2] },
        }
    }

    fn translate(&self, value: usize) -> Option<usize> {
        if !self.source_range.contains(&value) {
            return None;
        }

        let index_into_source_range = value - self.source_range.start;
        return Some(self.destination_range.start + index_into_source_range);
    }
}

#[derive(Debug, PartialEq)]
struct InformationMapping {
    source_type: InformationType,
    destination_type: InformationType,
    rules: Vec<MappingRule>,
}

impl InformationMapping {
    fn from_text(text: &str)  -> Self {
        let lines : Vec<&str> = text.split('\n').filter(|t| !t.is_empty()).map(|t| t.trim()).collect();

        let mapping_name_parts : Vec<&str> = lines[0].split(' ').filter(|t| !t.is_empty()).nth(0).unwrap().split('-').collect();
        let source_type = InformationType::from_string(mapping_name_parts[0]);
        let destination_type = InformationType::from_string(mapping_name_parts[2]);
        let mut rules : Vec<MappingRule> = Vec::new();

        for line in lines.iter().skip(1) {
            if line.is_empty() {
                continue;
            }
            rules.push(MappingRule::from_string(line));
        }
        
        Self {
            source_type,
            destination_type,
            rules
        }
    }

    fn translate(&self, value: usize) -> usize {
        for rule in &self.rules {
            match rule.translate(value) {
                Some(r) => return r,
                None => continue
            };
        }
        value
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    mappings: Vec<InformationMapping>
}

impl Almanac {
    fn from_string(text: &str) -> Self {
        let text_blocks : Vec<&str> = text.split("\n\n").collect();

        let seeds : Vec<usize> = text_blocks[0].split(':').nth(1).unwrap().trim().split(' ').map(|t| t.parse::<usize>().unwrap()).collect();
        let mappings : Vec<InformationMapping> = text_blocks.iter().skip(1).map(|text| InformationMapping::from_text(text)).collect();

        Self {
            seeds,
            mappings
        }
    }
}

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let almanac = Almanac::from_string(&text);

    let mut locations : Vec<usize> = Vec::new();

    for seed in almanac.seeds {
        
        let mut destination_value = seed;

        for mapping in &almanac.mappings {
            destination_value = mapping.translate(destination_value);
        }
        locations.push(destination_value);
    }
    println!("Lowest Destination: {:?}", locations.iter().min().unwrap());


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
