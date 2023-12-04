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
enum Spot {
    Number(String),
    Symbol(String),
    Empty,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coord {
    r: usize,
    c: usize
}

impl Coord {
    fn new(r: usize, c: usize) -> Self {
        Coord {
            r,
            c
        }
    }
}

fn build_engine(contents : String) -> Vec<Vec<Spot>> {
    contents.lines()
        .map(|line| line.chars())
        .map(|chars| chars.map(|c| {
            match c {
                c if c.is_digit(10) => {
                    Spot::Number(c.to_string())
                },
                c if c =='.' => {
                    Spot::Empty
                },
                _ => {
                    Spot::Symbol(c.to_string())
                }
            }
        }).collect())
        .collect()
}

fn get_num(
    r: usize, 
    c: usize, 
    engine: &Vec<Vec<Spot>>, 
    checked: &mut HashSet<Coord>
) -> u32 {
    let mut digit = String::new();

    let mut c_index = c;

    while c_index >= 0 && !checked.contains(&Coord::new(r, c_index)) {
        match &engine[r][c_index] {
            Spot::Number(val) => {
                checked.insert(Coord::new(r, c_index));
                digit.insert_str(0, &val);
            },
            _ => break,
        }
        

        // usize can't be -1 in Rust which is nice but makes this dumb
        if c_index == 0 {
            break;
        }
        c_index -= 1;
    }


    // Short circuit if don't need to check forwards
    if digit.is_empty() {
        return 0;
    }

    let mut c_index_forward = c + 1;
    while c_index_forward < engine[r].len() && !checked.contains(&Coord::new(r, c_index_forward)) {
        match &engine[r][c_index_forward] {
            Spot::Number(val) => {
                checked.insert(Coord::new(r, c_index_forward));
                digit.push_str(&val);
            },
            _ => break,
        }

        c_index_forward += 1;
    }
    
    // Guaranteed to be of some size due to the short circuit check above
    digit.parse::<u32>().unwrap()
}


fn get_part_numbers(
    r: usize, 
    c: usize, 
    engine: &Vec<Vec<Spot>>, 
    checked: &mut HashSet<Coord>
) -> Vec<u32> {
    let mut nums = Vec::new();

    nums.push(get_num(r-1, c-1, engine, checked));
    nums.push(get_num(r-1, c, engine, checked));    
    nums.push(get_num(r-1, c+1, engine, checked));    
    nums.push(get_num(r, c-1, engine, checked));    
    nums.push(get_num(r, c+1, engine, checked));
    nums.push(get_num(r+1, c-1, engine, checked));    
    nums.push(get_num(r+1, c, engine, checked));    
    nums.push(get_num(r+1, c+1, engine, checked));    

    nums.into_iter().filter(|num| num != &(0 as u32)).collect()
}

fn puzzle_1(contents: String) {
    let engine : Vec<Vec<Spot>> = build_engine(contents);

    let mut sum = 0;
    let mut checked = HashSet::new();

    for r in 0..engine.len() {
        for c in 0..engine[r].len() {
            let curr = &engine[r][c];

            match curr {
                Spot::Symbol(_) => sum += get_part_numbers(r, c, &engine, &mut checked)
                    .iter()
                    .sum::<u32>(),
                _ => {}
            }
            
        }
    }

    println!("The sum is: {}", sum);
}

fn puzzle_2(contents: String) {
    let engine : Vec<Vec<Spot>> = build_engine(contents);

    let mut sum = 0;
    let mut checked = HashSet::new();

    for r in 0..engine.len() {
        for c in 0..engine[r].len() {
            let curr = &engine[r][c];

            match curr {
                Spot::Symbol(val) => {
                    if val == "*" {
                        let part_numbers = get_part_numbers(r, c, &engine, &mut checked);

                        println!("Len: {}, part_numbers: {:?}", part_numbers.len(), part_numbers);

                        if part_numbers.len() == 2 {
                            sum += part_numbers.iter().product::<u32>();
                        } 
                    }
                }
                _ => {}
            }

        }
    }

    println!("The sum is: {}", sum);
}
