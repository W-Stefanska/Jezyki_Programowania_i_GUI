use std::io;
use rand::seq::SliceRandom;
use colored::Colorize;
struct Wordle {
    target_word: String,
    input_word: String,
    attempts: i32,
    max_attempts: i32
}

const WORDS: &[&str] = &["ocean", "adres", "koder", "input", "kiosk", "klucz", "login", "kwant", "szyfr", "delta"];
fn main() {
    let mut exit= false;
    while !exit {
        let target_word = (WORDS.choose(&mut rand::thread_rng())).unwrap().to_string();

        let mut game = Wordle{
            target_word,
            input_word: "".to_string(),
            attempts: 0,
            max_attempts: 6,
        };
        
        println!("Witaj w grze Wordle! Wpisz pierwsze slowo:\n");
        
        guessing_loop(&mut game);

        new_game(&mut exit);
    }
}

fn new_game (flag: &mut bool) {
    let mut check = String::new();
    println!("Czy chcesz zagrac ponownie? [t/n]\n");
    io::stdin().read_line(&mut check).expect("Blad odczytu");
    match check.trim().to_lowercase().as_str() {
        "n" => *flag = true,
        _ => (),
    }
}

fn format_guess(guess: &String, answer: &String) -> String {
    let mut result = String::new();
    let answer_chars: Vec<char> = answer.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();
    
    for (i, &ch) in guess_chars.iter().enumerate() {
        if answer_chars.get(i) == Some(&ch) {
            result.push_str(&format!("{}", ch.to_string().green()));
        } else if answer_chars.contains(&ch) {
            result.push_str(&format!("{}", ch.to_string().yellow()));
        } else {
            result.push_str(&format!("{}", ch.to_string().strikethrough()));
        }
    }
    return result;
}

fn wrong_input(game: &mut Wordle) {
    while game.input_word.len() != 5 || !game.input_word.chars().all(|x| x.is_alphabetic()) {
        println!("Wpisales: {}. Slowo musi skladac sie z 5 znakow i zawierac tylko litery.\n", game.input_word);
        game.input_word = String::new();
        io::stdin().read_line(&mut game.input_word).expect("Blad odczytu");
        game.input_word = game.input_word.trim().to_string();
        }
}

fn guessing_loop(game: &mut Wordle) {
    game.input_word = String::new();
    io::stdin().read_line(&mut game.input_word).expect("Blad odczytu");
    game.input_word = game.input_word.trim().to_string();
        
    while game.input_word != game.target_word {
        if game.attempts == game.max_attempts {
            break;
        }
        wrong_input(game);
        println!("{}", format_guess(&game.input_word, &game.target_word));
        game.input_word = String::new();
        io::stdin().read_line(&mut game.input_word).expect("Blad odczytu");
        game.input_word = game.input_word.trim().to_string();
        game.attempts += 1;
    }

    if game.input_word == game.target_word {
        println!("Wygrales! Ukryte slowo to: {}", game.target_word.green());
    }
    else {
        println!("Przegrales... Ukryte slowo to: {}", game.target_word);
        
    }
}