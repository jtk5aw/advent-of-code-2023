use std::env;
use std::fs;
use std::cmp::max;
use std::iter::Iterator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let puzzle_num : &i64 = &args[1].parse().unwrap();
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    if *puzzle_num == 1 as i64 {
        puzzle_1(contents);
    } else if *puzzle_num == 2 as i64 {
        puzzle_2(contents);
    } else {
        println!("bad puzzle num");
    }
}

#[derive(Debug)]
struct Game {
    game_id: i64,
    draws: Vec<Draw>
}

#[derive(Debug)]
struct Draw {
    blue: i64,
    green: i64,
    red: i64
}

fn parse_game(line: &str) -> Game {
    let mut game_and_list_of_draws = line.split(":");

    let game = game_and_list_of_draws.next().unwrap();
    let game_id = game.split(" ").last().unwrap().parse().unwrap();

    let draws = game_and_list_of_draws.last().unwrap().split(";")
        .map(|draw| {
            let counts = draw.split(",");

            let mut curr_draw = Draw {
                blue: 0,
                green: 0,
                red: 0
            };

            for count in counts {
                let mut num_and_color = count.trim().split(" ");

                let num = num_and_color.next().unwrap().parse().unwrap();
                let color = num_and_color.last().unwrap();

                match color { 
                    "blue" => {
                        curr_draw.blue = num;
                    },
                    "green" => {
                        curr_draw.green = num;
                    }, 
                    "red" => {
                        curr_draw.red = num;
                    }
                    _ => {
                        panic!("this color can't be procseed you should crash. Color in question: {}", color);
                    }
                }
            }

            curr_draw
        })
        .collect();

        Game {
            game_id,
            draws
        }
}

fn puzzle_1(contents: String) {
    let mut sum = 0;
    for line in contents.lines() {
        let game = parse_game(line);

        let bad_draw = game.draws.iter().find(|draw| {
            if draw.blue > 14 || draw.green > 13 || draw.red > 12 {
                return true;
            }
            false
        });

        if bad_draw.is_none() {
            sum += game.game_id;
        }
    }
    println!("Sum: {}", sum);
}

fn puzzle_2(contents: String) {
    let mut sum = 0;
    for line in contents.lines() {
        let game = parse_game(line);
        
        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for draw in game.draws {
            max_blue = max(max_blue, draw.blue);
            max_green = max(max_green, draw.green);
            max_red = max(max_red, draw.red);
        }

        sum += (max_blue * max_green * max_red);
    }
    println!("Sum: {}", sum);
}
