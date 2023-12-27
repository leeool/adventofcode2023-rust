use std::collections::HashMap;
use std::fs;

const COLOR_1: &str = "red";
const COLOR_2: &str = "green";
const COLOR_3: &str = "blue";

type Game = (u32, HashMap<String, u32>);

fn main() {
    let input = fs::read_to_string("./src/input").expect("should read the file");
    let lines: Vec<&str> = input.split("\n").collect();
    let games = &lines[..lines.len() - 1];

    let games_filtered = games.iter().map(|l| sum_cubes(l)).collect::<Vec<Game>>();
    let valid_games = possible_games(games_filtered);

    let sum: u32 = valid_games.iter().sum();
    println!("{}", sum)
}

fn sum_cubes(game: &str) -> Game {
    let colors = vec![COLOR_1.to_string(), COLOR_2.to_string(), COLOR_3.to_string()];
    let full_game: Vec<&str> = game.split(":").collect();
    let game_num: u32 = full_game[0].split(" ").collect::<Vec<_>>()[1]
        .parse::<u32>()
        .unwrap();
    let mut game_result: HashMap<String, u32> = colors
        .iter()
        .cloned()
        .zip([0_u32, 0_u32, 0_u32].iter().cloned())
        .collect::<HashMap<String, u32>>();
    let game_colors_filtered = full_game[1]
        .replace(" ", "")
        .replace(",", " ")
        .replace(";", " ");
    let game_colors = game_colors_filtered
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    for (i, c) in game_colors.iter().enumerate() {
        let (value, color) = split_number_and_string(c.to_string());
        let mut max_value: u32 = *game_result.get_key_value(&color.to_string()).unwrap().1;

        while i < game_colors.len() {
            max_value = if value > max_value { value } else { max_value };
            break 
        }

        game_result.insert(color.to_string(), max_value);
    }

    (game_num, game_result)
}

fn possible_games(games: Vec<Game>) -> Vec<u32> {
    let mut valid_games: Vec<u32> = Vec::new();

    for (n, g) in games {
        let g_red = g.get_key_value(COLOR_1).unwrap().1;
        let g_green = g.get_key_value(COLOR_2).unwrap().1;
        let g_blue = g.get_key_value(COLOR_3).unwrap().1;

        if g_green <= &13 && g_red <= &12 && g_blue <= &14 {
            valid_games.push(n);
        }
    }

    valid_games
}

fn split_number_and_string(s: String) -> (u32, String) {
    let mut string = String::new();
    let mut value = String::new();
    s.chars().for_each(|c| match c.is_digit(10) {
        true => value.push(c),
        false => string.push(c),
    });

    (value.parse::<u32>().unwrap(), string)
}
