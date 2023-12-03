use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
enum CubeColor {
    Red,
    Green,
    Blue
}

impl CubeColor {
    fn from_string(text: &str) -> Self {
       if text == "red" {
           CubeColor::Red
       } else if text == "green" {
           CubeColor::Green
       } else if text == "blue" {
           CubeColor::Blue
       } else {
            panic!("Unexpected COLOR");
       } 
    }
}


#[derive(Debug)]
struct CubeDraw {
    cubes: HashMap<CubeColor, usize>
}

#[derive(Debug)]
struct Game {
    id: usize,
    cube_draws: Vec<CubeDraw>
}

impl CubeDraw {
    fn from_string(text : &str) -> Self {
        let mut cubes : HashMap<CubeColor, usize> = HashMap::default();

        let draw_infos : Vec<&str> = text.split(",").collect();
        for draw_info in draw_infos {
            let draw_components : Vec<&str> = draw_info.split(" ").filter(|&x| !x.is_empty()).collect();
            let count = draw_components[0].parse::<usize>().unwrap();
            let color = CubeColor::from_string(draw_components[1]);
            cubes.insert(color, count);
        }


        Self {
            cubes
        }
    }
}

impl Game {
    fn from_string(text: &str) -> Result<Self, ()> {
        let game_parts  : Vec<&str>  = text.split(":").collect();
        if game_parts.len() != 2 {
            return Err(());
        }

        let game_id = game_parts[0][5..].parse::<usize>().unwrap();
        let cube_draws : Vec<CubeDraw> = game_parts[1].split(";").map(|x: &str| 
            CubeDraw::from_string(x)
        ).collect();
    

        return Ok(Self {
            id: game_id,
            cube_draws
        });
    }
}


fn riddle_part_one(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");
    let lines : Vec<&str> = text.split('\n').collect();


    let mut bag_loadout : HashMap<CubeColor, usize> = HashMap::new();
    bag_loadout.insert(CubeColor::Red, 12);
    bag_loadout.insert(CubeColor::Green, 13);
    bag_loadout.insert(CubeColor::Blue, 14);

    let mut game_id_sum = 0;

    for line in lines {
        let game = Game::from_string(line).expect("Error parsing game");
        let mut is_possible = true;

        'outer: for draw in game.cube_draws {
            for (cube_color, cube_count) in draw.cubes {
                if cube_count > bag_loadout[&cube_color] {
                    is_possible = false;
                    break 'outer;
                }
            }
        }

        if is_possible {
            game_id_sum += game.id;
        }
        
    }

    println!("Sum of all possible games: {:?}", game_id_sum);


}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");
    let lines : Vec<&str> = text.split('\n').collect();

    let mut cube_power_sum = 0;

    for line in lines {
        let game = Game::from_string(line).expect("Error parsing game");

        let mut min_count_per_cube_color : HashMap<CubeColor, usize> = HashMap::new();

        for draw in game.cube_draws {
            for (cube_color, cube_count) in draw.cubes {
               if cube_count > *min_count_per_cube_color.get(&cube_color).unwrap_or(&0) {
                    min_count_per_cube_color.insert(cube_color, cube_count);
               }
            }
        }

        let mut cube_power = 1;
        for (cube_color, cube_count) in min_count_per_cube_color {
            cube_power *= cube_count;
        }
        cube_power_sum += cube_power;
    }

    println!("Cube power sum: {:?}", cube_power_sum);

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