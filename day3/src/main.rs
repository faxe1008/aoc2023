use std::collections::HashSet;
use std::fs;
use std::env;

const SYMBOLS : [char; 11] = ['%', '+', '#', '&', '$', '*', '\n', '=', '-', '@', '/'];

const OFFSETS : [Coord2D; 8] = [
    Coord2D {x: -1, y: -1}, Coord2D {x: 0, y: -1}, Coord2D {x: 1, y: -1},
    Coord2D {x: -1, y: 0},  Coord2D {x: 1, y: 0},
    Coord2D {x: -1, y: 1}, Coord2D {x: 0, y: 1}, Coord2D {x: 1, y: 1},
];

#[derive(Hash, PartialEq, Debug, Eq, Clone)]
struct Coord2D {
    x: isize,
    y: isize
}

#[derive(Debug, PartialEq, Eq,)]
struct EnginePart {
    number: u32,
    digit_coords: HashSet<Coord2D>
}

#[derive(Debug)]
struct EngineSchematic {
    width: usize,
    height: usize,

    parts: Vec<EnginePart>,
    symbols: HashSet<Coord2D>
}

impl EngineSchematic {

    fn from_string(text: &str, symbols_chars: &[char]) -> Self {
        let rows : Vec<&str> = text.split('\n').collect();

        let width = rows[0].len();
        let height = rows.len();
    
        let mut engine_part_list : Vec<EnginePart> = Vec::new();
        let mut symbol_list : HashSet<Coord2D> = HashSet::new();
    
        for (row_index, row_text) in rows.iter().enumerate() {
    
            let digit_indicies : Vec<(usize, &str)> = row_text.match_indices(|c: char| c.is_digit(10)).collect();
            
            let mut number_buffer = String::new();
            let mut digit_coords : HashSet<Coord2D> = HashSet::new();
            let mut last_digit_pos : Option<usize> = None;
    
            for (digit_pos, digit_slc) in digit_indicies {
                if last_digit_pos == None || last_digit_pos.unwrap() + 1 == digit_pos  {
                    number_buffer.push_str(digit_slc);
                    last_digit_pos = Some(digit_pos);
                } else {                
                    let engine_part = EnginePart {
                        number: number_buffer.parse::<u32>().unwrap(),
                        digit_coords: digit_coords.clone()
                    };
                    engine_part_list.push(engine_part);
    
                    digit_coords.clear();
                    number_buffer = digit_slc.to_string();
                    last_digit_pos = Some(digit_pos)
                }
                digit_coords.insert(Coord2D { x: digit_pos as isize, y: row_index as isize});
    
            }
    
            if number_buffer.len() > 0 {
                let engine_part = EnginePart {
                    number: number_buffer.parse::<u32>().unwrap(),
                    digit_coords: digit_coords.clone()
                };
                engine_part_list.push(engine_part);
            }
    

            let symbol_coords : HashSet<Coord2D> = row_text.match_indices(|c: char| symbols_chars.contains(&c)).map(|(pos, _s)| Coord2D {x: pos as isize, y:row_index as isize}).collect();
            for cr in symbol_coords {
                symbol_list.insert(cr);
            }
        }   
    
        Self {
            width,
            height,
            parts: engine_part_list,
            symbols: symbol_list
        }
    }


    fn get_valid_engine_parts_sum(&self) -> u32 {

      
        let mut part_sum = 0;

        for engine_part in &self.parts {

            'loop_digits: for digit_coords in &engine_part.digit_coords {

                for offs in &OFFSETS {
                    let potential_symbol_pos = Coord2D { x: digit_coords.x + offs.x, y: digit_coords.y + offs.y };
                    
                    if self.symbols.contains(&potential_symbol_pos) {
                        part_sum += engine_part.number;
                        break 'loop_digits;
                    }

                }

            }

        }
        part_sum
    }


    fn get_gear_ratio(&self) -> u32 {
        // at this point symbols only contains the gears

        let mut gear_ratio = 0;

        for gear in &self.symbols {
            let mut neighbouring_engine_parts : Vec<&EnginePart> = Vec::new();

            
            for offs in &OFFSETS {
                let potential_pos = Coord2D { x : gear.x + offs.x, y:gear.y + offs.y};

                let mut neighbors : Vec<&EnginePart> = self.parts.iter().filter(|part| part.digit_coords.contains(&potential_pos)).collect();
               
                for neigbor in neighbors {
                    if !neighbouring_engine_parts.contains(&neigbor) {
                        neighbouring_engine_parts.push(neigbor);
                    }
                }
            }
           
            if neighbouring_engine_parts.len() == 2 {
                gear_ratio += (neighbouring_engine_parts[0].number * neighbouring_engine_parts[1].number);
            }

        }
        gear_ratio
    }
}

fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let schematic = EngineSchematic::from_string(&text, &SYMBOLS);    
    let part_sum = schematic.get_valid_engine_parts_sum();
    println!("{:?}", part_sum);
}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let gears = ['*'];
    let schematic = EngineSchematic::from_string(&text, &gears);  
    let gear_ratio = schematic.get_gear_ratio();
    println!("{:?}", gear_ratio);
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