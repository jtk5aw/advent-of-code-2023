use std::env;
use std::fs;
use std::collections::HashSet;

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
struct Card {
    winning: HashSet<String>,
    hand: HashSet<String>
}

fn parse_card(line: String) -> Card {
    let game = line.split(":").last().expect("Input will have one :");

    let (winning, hand) = game.split_once("|").expect("Has to have |");

    let winning_set : HashSet<String> = HashSet::from_iter(
        winning.trim().split_whitespace().map(|str| str.to_owned())
    );
    let hand_set : HashSet<String> = HashSet::from_iter(
        hand.trim().split_whitespace().map(|str| str.to_owned())
    );

    Card {
        winning:  winning_set,
        hand: hand_set
    }
}

fn puzzle_1(contents: String) {
    let mut sum = 0;

    for line in contents.lines() {

        let card = parse_card(line.to_string());
        
        let intersection = card.winning.intersection(&card.hand);
        let intersection_count = intersection.count();

        if intersection_count > 0 {
            sum += 2_i32.pow((intersection_count - 1) as u32);
        }
    }


    println!("Sum is {}", sum);
}

fn puzzle_2(contents: String) {
    let lines : Vec<&str> = contents.lines().collect();
    let mut result = vec![1; lines.len()];

    for line_num in 0..lines.len() {
        let card = parse_card(lines[line_num].to_string());

        let intersection = card.winning.intersection(&card.hand);
        let intersection_count = intersection.count();

        for i in 1..=intersection_count {
            result[line_num + i] += 1 * result[line_num];
        }
    }
    
    let sum = result.iter().sum::<u32>(); 

    println!("Sum is {}", sum);
}
