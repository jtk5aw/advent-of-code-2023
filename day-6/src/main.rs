use std::env;
use std::fs;

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

fn perform_quadratic(time: &f64, distance: &f64) -> (f64, f64) {
    let sqrt = (time.powf(2.0) - 4.0 * distance).sqrt();
    ((time - sqrt)/2.0, (time + sqrt)/2.0)
}

fn puzzle_1(contents: String) {
    let mut lines = contents.lines();

    let times : Vec<f64> = lines.next().expect("has to have two lines")
        .split(":").last().expect("has to have the time: at the front")
        .split_whitespace()
        .map(|num| num.parse::<f64>().expect("has to be a number"))
        .collect();
    let distances : Vec<f64> = lines.next().expect("has to have two lines")
        .split(":").last().expect("has to have the distace: at the front")
        .split_whitespace()
        .map(|num| num.parse::<f64>().expect("has to be a number"))
        .collect();

    let mut result = 1.0;

    for i in 0..times.len() {
        let (first_root, second_root) = perform_quadratic(&times[i], &distances[i]);
        println!("first root:  {:?}, second root: {:?}", first_root, second_root);
        let first_root_ceil = if first_root.ceil() == first_root { 
            first_root + 1.0 
        } else { 
            first_root.ceil() 
        };
        let second_root_floor = if second_root.floor() != second_root {
            second_root.floor() + 1.0
        } else {
            second_root.floor()
        };
        println!("first root ceil: {:?}, second root floor: {:?}", first_root_ceil, second_root_floor); 
        result *= second_root_floor - first_root_ceil;
        println!("new result: {:?}", result);
        println!("");
    }

    println!("the result is: {:?}", result);

}

fn puzzle_2(contents:String) {
    let mut lines = contents.lines();
    
    let mut time_str : String = "".to_string();
    lines.next().expect("has to have two lines")
        .split(":").last().expect("has to have the time: at the front")
        .split_whitespace()
        .for_each(|num| time_str.push_str(num));
    let time = time_str.parse::<f64>().expect("Only added digits");

    let mut distance_str : String = "".to_string();
    lines.next().expect("has to have two lines")
        .split(":").last().expect("has to have the distace: at the front")
        .split_whitespace()
        .for_each(|num| distance_str.push_str(num));
    let distance = distance_str.parse::<f64>().expect("only added digits");

    let (first_root, second_root) = perform_quadratic(&time, &distance);
    println!("first root:  {:?}, second root: {:?}", first_root, second_root);
    let first_root_ceil = if first_root.ceil() == first_root { 
        first_root + 1.0 
    } else { 
        first_root.ceil() 
    };
    let second_root_floor = if second_root.floor() != second_root {
        second_root.floor() + 1.0
    } else {
        second_root.floor()
    };
    println!("first root ceil: {:?}, second root floor: {:?}", first_root_ceil, second_root_floor); 
    let result = second_root_floor - first_root_ceil;
    println!("new result: {:?}", result);
    println!("");

    println!("the result is: {:?}", result);

}

