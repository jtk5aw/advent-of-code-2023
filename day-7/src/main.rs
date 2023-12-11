use std::env;
use std::fs;
use std::cmp::Ordering;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    card_ranks: Vec<u64>,
    bid:  u64
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for i in 0..self.card_ranks.len() {
            if self.card_ranks[i] != other.card_ranks[i] {
                return self.card_ranks[i].cmp(&other.card_ranks[i]);
            }
        }

        Ordering::Equal
    }
}

fn get_rank(c: char) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        val => {
          val.to_digit(10).expect("has to be a digit").into()
        }
    }
}

fn get_rank_with_joker(c: char) -> u64{
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0, // Jokers are worth the least 
        'T' => 10,
        val => {
          val.to_digit(10).expect("has to be a digit").into()
        }
    }
}

fn get_hand_type(runs: &Vec<u64>) -> HandType {
    match runs.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if runs[0] == 4 || runs[1] == 4 {
                return HandType::FourOfAKind;
            }
            HandType::FullHouse
        },
        3 => {
            if runs[0] == 3 || runs[1] == 3 || runs[2] == 3 {
                return HandType::ThreeOfAKind;
            }
            HandType::TwoPair
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => { 
            println!("Num runs: {:?}", runs); 
            panic!("This should never be reached"); 
        }
    }
}

fn get_hand_type_with_joker(runs: &Vec<u64>) -> HandType {
    let sum : u64 = runs.iter().sum();

    if sum == 5 {
        return get_hand_type(runs);
    }

    match runs.len() {
        0 => HandType::FiveOfAKind,
        1 => HandType::FiveOfAKind,
        2 => {
            if sum == 2 || sum == 3 || (sum == 4 && (runs[0] == 3 || runs[1] == 3)) {
               return HandType::FourOfAKind; 
            }
            HandType::FullHouse
        },
        3 => HandType::ThreeOfAKind,
        4 => HandType::OnePair,
        _ => { 
            println!("Num runs: {:?}", runs); 
            panic!("This should never be reached"); 
        }
    }
}

fn parse_hand(line: &str) -> Hand {
    let (cards, bid_str) = line.split_once(" ").expect("Should be cards and big");
    let bid = bid_str.parse::<u64>().expect("Should be a num");

    let mut card_ranks : Vec<u64> = Vec::new();

    for card in cards.chars() {
       card_ranks.push(get_rank(card));
    }

    let mut cards_sorted = cards.chars().collect::<Vec<char>>();
    cards_sorted.sort();
    let mut runs = Vec::new();

    let mut prev_char = cards_sorted[0];
    let mut curr_count = 1;

    for i in 1..cards_sorted.len() {
        if prev_char == cards_sorted[i] {
            curr_count += 1;
        } else {
            runs.push(curr_count);
            prev_char = cards_sorted[i];
            curr_count = 1;
        }
    }
    runs.push(curr_count);
    curr_count = 0;

    let hand_type = get_hand_type(&runs);

    Hand {
        hand_type,
        card_ranks,
        bid
    }
}

fn parse_hand_with_joker(line: &str) -> Hand {
    let (cards, bid_str) = line.split_once(" ").expect("Should be cards and big");
    let bid = bid_str.parse::<u64>().expect("Should be a num");

    let mut card_ranks : Vec<u64> = Vec::new();

    for card in cards.chars() {
       card_ranks.push(get_rank_with_joker(card));
    }

    let mut cards_sorted = cards.chars().filter(|c| c != &'J').collect::<Vec<char>>();
    cards_sorted.sort();
    let mut runs = Vec::new();

    if cards_sorted.len() != 0 {
        let mut prev_char = cards_sorted[0];
        let mut curr_count = 1;

        for i in 1..cards_sorted.len() {
            if prev_char == cards_sorted[i] {
                curr_count += 1;
            } else {
                runs.push(curr_count);
                prev_char = cards_sorted[i];
                curr_count = 1;
            }
        }
        runs.push(curr_count);
        curr_count = 0;
    }

    let hand_type = get_hand_type_with_joker(&runs);

    Hand {
        hand_type,
        card_ranks,
        bid
    }
}

fn puzzle_1(contents: String) {
    let mut hands : Vec<Hand> = contents.lines()
        .map(|line| parse_hand(line))
        .collect();

    hands.sort();

    let mut result : u64 = 0;

    for i in 0..hands.len() {
        let val = (i as u64 + 1) * hands[i as usize].bid; 
        result += val;
    }

    println!("The result is {:?}", result);

}

fn puzzle_2(contents: String) {
    let mut hands : Vec<Hand> = contents.lines()
        .map(|line| parse_hand_with_joker(line))
        .collect();

    hands.sort();

    let mut result : u64 = 0;

    for i in  0..hands.len() {
        let val = (i as u64 + 1) * hands[i as usize].bid; 
        result += val;
    }

    println!("The result is {:?}", result);
}
