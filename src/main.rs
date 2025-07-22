mod file_reader;

use file_reader::*;
use std::{io, time::Instant};

fn main() {
    start_notification();
    choosing_mode();
}

fn start_notification() {
    clear_screen();
    println!("Typing Trainer");
    println!("Do you want to start?");
    println!("[1] | Start");
    println!("[2] | Exit");
}

fn exit() {
    println!("Bye!");
    std::process::exit(0);
}

fn choosing_mode() {
    let input = read_input();
    if input == "1" {
        println!("Starting...");
        get_started();
    } else if input == "2" {
        println!("Exiting...");
        exit();
    } else {
        println!("Invalid input");
        start_notification();
    }
}

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => println!("Error reading input: {}", error),
    }
    input.trim().to_string() // <--- Ось це прибирає \n
}

fn get_started() {
    let sentences = read_file("sentences.txt");
    if sentences.len() == 0 {
        println!("No sentences found. Add sentences to 'sentences.txt' file.");
        println!("Exiting...");
        exit();
    }

    let sentences = parse_file(sentences);
    loop {
        let current_sentence = get_random_sentence(&sentences);
        println!("{}", current_sentence);
        let start = Instant::now();
        let input = read_input();

        let duration = start.elapsed();
        let (correct, incorrect) = check_correct_and_incorrect(current_sentence, input);
        println!("Correct: {} | Incorrect: {}", correct, incorrect);
        println!("Time: {} seconds", duration.as_secs());

        println!("Pause at 5 seconds...");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn check_correct_and_incorrect(sentence: String, input: String) -> (u8, u8) {
    let mut correct: u8 = 0;
    let mut incorrect: u8 = 0;

    let sentence: Vec<char> = sentence.chars().collect();
    let input: Vec<char> = input.chars().collect();

    let len = sentence.len().min(input.len());

    for i in 0..len {
        if sentence[i] != input[i] {
            incorrect = incorrect + 1;
        } else {
            correct = correct + 1;
        }
    }

    return (correct, incorrect);
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
