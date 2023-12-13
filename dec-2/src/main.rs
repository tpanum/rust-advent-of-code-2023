use regex::Regex;
use std::fs::File;
use std::io::BufRead;

struct Game {
    id: u8,
    red_cubes: Vec<u8>,
    blue_cubes: Vec<u8>,
    green_cubes: Vec<u8>,
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let reader = std::io::BufReader::new(f);

    let game_re = Regex::new(r"^Game (\d+)").unwrap();
    let cube_re = Regex::new(r"(\d+) (green|red|blue)").unwrap();
    let mut games: Vec<Game> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let cap = game_re.captures(&line).unwrap();
        let mut game = Game {
            id: 0,
            blue_cubes: Vec::new(),
            green_cubes: Vec::new(),
            red_cubes: Vec::new(),
        };
        match cap.get(1) {
            Some(v) => {
                let game_id = v.as_str().parse::<u8>().unwrap();
                game.id = game_id;
            }
            None => {}
        }

        cube_re.captures_iter(&line).for_each(|cap| {
            let n_str = cap.get(1).unwrap().as_str();
            let n = n_str.parse::<u8>().unwrap();
            let color = cap.get(2).unwrap().as_str();

            match color {
                "blue" => game.blue_cubes.push(n),
                "red" => game.red_cubes.push(n),
                "green" => game.green_cubes.push(n),
                &_ => {}
            }
        });

        games.push(game);
    }

    let mut total_id: u32 = 0;
    let mut total_power_sum: u32 = 0;

    for g in games {
        let max_green = u32::from(g.green_cubes.iter().max().unwrap().to_owned());
        let max_red = u32::from(g.red_cubes.iter().max().unwrap().to_owned());
        let max_blue = u32::from(g.blue_cubes.iter().max().unwrap().to_owned());

        let is_valid_game = max_red <= 12 && max_green <= 13 && max_blue <= 14;
        if is_valid_game {
            total_id += u32::from(g.id);
        }

        total_power_sum += max_green * max_red * max_blue;
    }

    println!("Total IDs (part 1): {total_id}");
    println!("Total Power Sum (part 2): {total_power_sum}");
}
