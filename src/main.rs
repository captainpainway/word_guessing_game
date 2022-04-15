mod words;
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
use chrono::{Utc, TimeZone};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut guesses = 1;
    let day = day_number();
    let word = word_of_the_day(&day);
    let mut guessed_words: Vec<String> = Vec::new();
    let mut emoji: Vec<String> = Vec::new();
    let mut alphabet: HashMap<String, &str> = HashMap::from([
        ("A".to_string(), "unused"), ("B".to_string(), "unused"), ("C".to_string(), "unused"), ("D".to_string(), "unused"),
        ("E".to_string(), "unused"), ("F".to_string(), "unused"), ("G".to_string(), "unused"), ("H".to_string(), "unused"),
        ("I".to_string(), "unused"), ("J".to_string(), "unused"), ("K".to_string(), "unused"), ("L".to_string(), "unused"),
        ("M".to_string(), "unused"), ("N".to_string(), "unused"), ("O".to_string(), "unused"), ("P".to_string(), "unused"),
        ("Q".to_string(), "unused"), ("R".to_string(), "unused"), ("S".to_string(), "unused"), ("T".to_string(), "unused"),
        ("U".to_string(), "unused"), ("V".to_string(), "unused"), ("W".to_string(), "unused"), ("X".to_string(), "unused"),
        ("Y".to_string(), "unused"), ("Z".to_string(), "unused")
    ]);

    println!("\nNOT THAT WORD GAME\n");
    loop {
        if guesses > 6 {
            println!("Out of guesses! The word was {}.", &word.to_uppercase());
            print_emojis(emoji, &day, &guesses);
            break;
        }
        println!("Guess #{:?}:", guesses);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");
        let guess_len = guess.trim().chars().count() as i32;

        // Check to see if the guess is in either word list.
        let guess_is_in_valid_words = words::VALID_WORDS
            .iter()
            .position(|&x| x == guess.trim().to_lowercase())
            .unwrap_or(!0) as i32;
        let guess_is_in_word_list = words::WORDS
            .iter()
            .position(|&x| x == guess.trim().to_lowercase())
            .unwrap_or(!0) as i32;

        // If both checks don't return a positive index, the word is not valid.
        if guess_is_in_valid_words == -1 && guess_is_in_word_list == -1 {
            println!("Not a valid word.");
        } else {
            match guess_len.cmp(&5) {
                Ordering::Equal => {
                    match compare_words(&word, &guess) {
                        true => {
                            create_emojis(&word, &guess, &mut emoji);
                            guessed_words.push(guess);
                            color_letters(&word, &guessed_words, &mut alphabet);
                            println!("You win in {} tries!", guesses);
                            print_emojis(emoji, &day, &guesses);
                            break;
                        },
                        _ => {
                            create_emojis(&word, &guess, &mut emoji);
                            guessed_words.push(guess);
                            color_letters(&word, &guessed_words, &mut alphabet);
                            color_alphabet(&alphabet);
                        }
                    }
                    guesses += 1;
                },
                _ => println!("Enter a 5-letter word.")
            }
        }
    }
}

fn day_number() -> usize {
    let start = Utc.ymd(2022, 04, 15).and_hms(0, 0, 0);
    let now = Utc::now();
    let duration: usize = (now - start).num_days() as usize;
    return duration;
}

fn word_of_the_day(day: &usize) -> String {
    return (words::WORDS)[*day].to_string();
}

fn compare_words(word: &String, guess: &String) -> bool {
    if word.eq(&guess.trim().to_lowercase()) {
        return true;
    }
    return false;
}

fn color_letters(word: &String, guessed_words: &Vec<String>, alphabet: &mut HashMap<String, &str>) -> bool {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    let word_vec: Vec<&str> = word.trim().split("").filter(|x| x.len() > 0).collect();

    write!(&mut stdout, "\n").unwrap();
    for n in 0..guessed_words.len() {
        let g = &guessed_words[n];
        let guess_vec: Vec<&str> = g.trim().split("").filter(|x| x.len() > 0).collect();

        for i in 0..5 {
            if guess_vec[i] == word_vec[i] {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                write!(&mut stdout, "{}", guess_vec[i].to_uppercase()).unwrap();

                // Update alphabet vector.
                alphabet.insert(guess_vec[i].to_uppercase(), "correct");
            } else if word_vec.contains(&guess_vec[i]) {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
                write!(&mut stdout, "{}", guess_vec[i].to_uppercase()).unwrap();

                // A correct letter should always be coded correct.
                if alphabet[&guess_vec[i].to_uppercase()] != "correct" {
                    alphabet.insert(guess_vec[i].to_uppercase(), "close");
                }
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
                write!(&mut stdout, "{}", guess_vec[i].to_uppercase()).unwrap();

                // Update alphabet vector.
                alphabet.insert(guess_vec[i].to_uppercase(), "wrong");
            }
            write!(&mut stdout, " ").unwrap();
        }
        write!(&mut stdout, "\n").unwrap();
    }

    write!(&mut stdout, "\n").unwrap();
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
    return true;
}

fn color_alphabet(alphabet: & HashMap<String, &str>) -> bool {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for key in alphabet.keys().sorted() {
        if alphabet[key] == "correct" {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
        } else if alphabet[key] == "close" {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        } else if alphabet[key] == "wrong" {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(100, 100, 100)))).unwrap();
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
        }
        write!(&mut stdout, "{}", key).unwrap();
        write!(&mut stdout, " ").unwrap();
    }
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();

    write!(&mut stdout, "\n\n").unwrap();
    return true;
}

fn create_emojis(word: &String, guess: &String, emoji: &mut Vec<String>) -> bool {
    let mut emoji_str = String::new();
    let word_vec: Vec<&str> = word.trim().split("").filter(|x| x.len() > 0).collect();
    let guess_vec: Vec<&str> = guess.trim().split("").filter(|x| x.len() > 0).collect();
    for i in 0..5 {
        if guess_vec[i] == word_vec[i] {
            emoji_str = emoji_str.to_owned() + "🟩";
        } else if word_vec.contains(&guess_vec[i]) {
            emoji_str = emoji_str.to_owned() + "🟨";
        } else {
            emoji_str = emoji_str.to_owned() + "⬛";
        }
    }
    emoji.push(emoji_str);
    return true;
}

fn print_emojis(emoji: Vec<String>, day: &usize, guesses: &i32) -> bool {
    println!("\n");
    let number_of_guesses = if *guesses > 6 {"X".to_string()} else {guesses.to_string()};
    println!("NOT THAT WORD GAME {} {}/6\n", day, number_of_guesses);
    for i in emoji.iter() {
        println!("{}", i);
    }
    println!("\n");
    return true;
}