use std::env;
use std::fs;
use std::str::Chars;
use std::collections::HashMap;

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

fn get_digit(mut chars: Chars) -> String {
    let first_digit = chars.find(|char| char.is_digit(10)).unwrap();
    let last_digit = match chars.rfind(|char| char.is_digit(10)) {
        Some(val) => val,
        None => first_digit
    };

    format!("{}{}", first_digit, last_digit)
}

fn puzzle_1(contents: String) { 
    let mut digits: Vec<i64> = Vec::new();

    for line in contents.lines() {
        let chars = line.chars();

        let digit = get_digit(chars);

        digits.push(digit.parse().unwrap());
    }

    println!("With digits: \n{:?}", digits);

    println!("Sum is: {}", digits.iter().sum::<i64>());
}

fn create_letter_map(digit_words: &Vec<String>) -> HashMap<u8, Vec<String>> {
    let digit_word_tuples : Vec<(u8, String)> = digit_words.iter()
        .map(|word| (word.as_bytes()[0], word.to_owned()))
        .collect();
    let first_letters : HashMap<u8, Vec<String>> = digit_word_tuples.into_iter().fold(HashMap::new(), |mut map, (k,v)| {
        map.entry(k)
            .and_modify(|map| map.push(v.to_owned()))
            .or_insert(vec![v.to_owned()]);
        map
    });
    first_letters 
}

fn search_for_word(start_index: usize, chars: &[u8], word: &[u8]) -> bool {
    let mut word_index : usize = 0;
    let mut index : usize = start_index;
    
    while index < chars.len() {
        if word_index == word.len() - 1 && word[word_index] == chars[index] {
            return true;
        } else if word[word_index] != chars[index] {
            return false;
        }

        index += 1;
        word_index += 1;
     }
    false
}

fn puzzle_2(contents: String) {
    let mut digits: Vec<i64> = Vec::new();

    let digit_words : Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string(), "seven".to_string(), "eight".to_string(), "nine".to_string()];
    let backwards_digit_words : Vec<String> = digit_words.iter().map(|word| word.chars().rev().collect::<String>()).collect();

    println!("Backwards: {:?}", backwards_digit_words);

    let mut digit_word_to_val : HashMap<String, String> = HashMap::new();

    for i in 0..digit_words.len() {
        digit_word_to_val.insert(digit_words[i].to_owned(), (i + 1).to_string());
        digit_word_to_val.insert(backwards_digit_words[i].to_owned(), (i + 1).to_string());
    }


    let first_letters = create_letter_map(&digit_words);
    let last_letters = create_letter_map(&backwards_digit_words);

    println!("first letters: {:?}", first_letters);
    println!("last letters: {:?}", last_letters);

    for line in contents.lines() {
        let chars = line.as_bytes();
        let backwards_line = line.chars().rev().collect::<String>();
        println!("Forwards line: {}", line);
        println!("Backwards line: {}", backwards_line);
        let backwards = backwards_line.as_bytes();

        let mut first_digit : Option<String> = None;
        let mut second_digit : Option<String> = None;
        
        for i in 0..chars.len() {
            // Forwards 
            if first_digit.is_none() && (&chars[i]).is_ascii_digit() {

                let char_arr = [chars[i].to_owned()];
                first_digit = Some(std::str::from_utf8(&char_arr).unwrap().to_owned());

            } else if first_digit.is_none() && first_letters.get(&chars[i]).is_some() {

                for word in first_letters.get(&chars[i]).unwrap() {
                    if search_for_word(i, chars, word.as_bytes()) {
                        first_digit = Some(digit_word_to_val.get(word).unwrap().to_owned());
                        break;
                    }
                }

            }

            // Backwards 
            if second_digit.is_none() && (&backwards[i]).is_ascii_digit() {

                let char_arr = [backwards[i].to_owned()];
                second_digit = Some(std::str::from_utf8(&char_arr).unwrap().to_owned());

            } else if second_digit.is_none() && last_letters.get(&backwards[i]).is_some() {

                for word in last_letters.get(&backwards[i]).unwrap() {
                    if search_for_word(i, backwards, word.as_bytes()) {
                        second_digit = Some(digit_word_to_val.get(word).unwrap().to_owned());
                        break;
                    }
                }

            }
        }

        let digit = format!("{}{}", first_digit.clone().unwrap(), second_digit.unwrap_or_else(|| first_digit.unwrap()));

        println!("Digit: {}", digit);

        digits.push(digit.parse().unwrap());
    }

    println!("With digits: \n{:?}", digits);

    println!("Sum is: {}", digits.iter().sum::<i64>());
}

