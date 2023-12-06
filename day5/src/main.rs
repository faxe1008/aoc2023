use core::panic;
use std::cmp;
use std::env;
use std::fs;
use std::ops::Range;
use std::result;

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
    source_range: Range<isize>,
    destination_range: Range<isize>,
}

impl MappingRule {
    fn from_string(text: &str) -> Self {
        // 0 15 37
        let numbers: Vec<isize> = text
            .trim()
            .split(' ')
            .filter(|t| !t.is_empty())
            .map(|t| t.parse::<isize>().expect("Error parsing number"))
            .collect();
        if numbers.len() != 3 {
            panic!("Nonsense rule found");
        }

        Self {
            destination_range: Range { start: numbers[0], end: numbers[0] + numbers[2] },
            source_range: Range { start: numbers[1], end: numbers[1] + numbers[2] },
        }
    }

    fn translate(&self, value: isize) -> Option<isize> {
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

trait Overlap<T> {
    fn overlap(&self, other: &Range<T>) -> Range<T>;
}

impl<T> Overlap<T> for Range<T> where T: Ord + Copy {
    fn overlap(&self, other: &Range<T>) -> Range<T> {
        if self.start < other.end && self.end >= other.start {
            Range {start: cmp::max(self.start, other.start), end: cmp::min(self.end, other.end)}
        } else {
            Range {start: self.start, end: self.start}
        }
    }
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

    fn translate(&self, value: isize) -> isize {
        for rule in &self.rules {
            match rule.translate(value) {
                Some(r) => return r,
                None => continue
            };
        }
        value
    }

    fn translate_range(&self, value_range: Range<isize>) -> Vec<Range<isize>> {

        let mut resulting_ranges = Vec::new();

        for rule in &self.rules {
            let overlap = value_range.overlap(&rule.source_range);
            if overlap.is_empty() {
                continue;
            }

            let res_start = rule.translate(overlap.start).unwrap();
            let res_end = rule.translate(overlap.end).unwrap();

            resulting_ranges.push(Range {start: res_start, end: res_end});
        }


        resulting_ranges
    }

}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<isize>,
    mappings: Vec<InformationMapping>
}

impl Almanac {
    fn from_string(text: &str) -> Self {
        let text_blocks : Vec<&str> = text.split("\n\n").collect();

        let seeds : Vec<isize> = text_blocks[0].split(':').nth(1).unwrap().trim().split(' ').map(|t| t.parse::<isize>().unwrap()).collect();
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

    let mut locations : Vec<isize> = Vec::new();

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

    let almanac = Almanac::from_string(&text);

    // Transform the seeds into their ranges
    let mut current_gen : Vec<Range<isize>> = Vec::new();
    for index in (0..almanac.seeds.len()).step_by(2) {
        let start  = almanac.seeds[index];
        let len = almanac.seeds[index + 1];
        current_gen.push(Range {start: start, end: start + len });
    }

    for convert in &almanac.mappings {
        
        let mut converted : Vec<Range<isize>> = Vec::new();
        let mut unconverted = current_gen.clone();

        for rule in &convert.rules {
            let convert_range = &rule.source_range;
            let offset = rule.destination_range.start - rule.source_range.start;

            let mut new_unconverted = Vec::<Range<isize>>::new();

            for r in &unconverted {
                let overlap = r.overlap(&convert_range);

                let left = Range { start: r.start, end: overlap.start};
                if left.end > left.start {
                    new_unconverted.push(left);
                }

                if overlap.end > overlap.start {
                    converted.push(Range { start: overlap.start + offset, end: overlap.end + offset});
                }

                let right = Range {start: overlap.end, end: r.end};
                if right.end > right.start {
                    new_unconverted.push(right);
                }

            }
            unconverted = new_unconverted;
        }
        current_gen = Vec::new();
        current_gen.append(&mut converted);
        current_gen.append(&mut unconverted);
    }

    dbg!(current_gen.iter().min_by(|x, y| x.start.cmp(&y.start)));
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
