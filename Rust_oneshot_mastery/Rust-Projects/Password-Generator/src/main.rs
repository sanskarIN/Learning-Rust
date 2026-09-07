// ------------------------------------------------------------------------------------------------------------------
// Hey Everyone I am going to create the simple project of the password generator in the Rust Programming Language.
// Let's start creating this project from totally zero to the hero.
// ------------------------------------------------------------------------------------------------------------------
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};

fn generate_password(length: usize, difficulty: u32) -> String {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let symbols = "!@#$%^&*()-_=+[]{}<>?/|~";

    let mut characters = String::new();
    let mut password = String::new();

    match difficulty {
        1 => {
            // Easy
            characters.push_str(lowercase);
            characters.push_str(numbers);
        }

        2 => {
            // Medium
            characters.push_str(lowercase);
            characters.push_str(uppercase);
            characters.push_str(numbers);
        }

        3 | 4 => {
            // Hard / Extreme
            characters.push_str(lowercase);
            characters.push_str(uppercase);
            characters.push_str(numbers);
            characters.push_str(symbols);
        }

        _ => return String::new(),
    }

    let mut rng = rand::rng();

    // Make sure each character type is represented.
    match difficulty {
        1 if length >= 2 => {
            password.push(random_char(lowercase, &mut rng));
            password.push(random_char(numbers, &mut rng));
        }

        2 if length >= 3 => {
            password.push(random_char(lowercase, &mut rng));
            password.push(random_char(uppercase, &mut rng));
            password.push(random_char(numbers, &mut rng));
        }

        3 | 4 if length >= 4 => {
            password.push(random_char(lowercase, &mut rng));
            password.push(random_char(uppercase, &mut rng));
            password.push(random_char(numbers, &mut rng));
            password.push(random_char(symbols, &mut rng));
        }

        _ => {}
    }

    // Fill the remaining characters.
    while password.len() < length {
        password.push(random_char(&characters, &mut rng));
    }

    // Shuffle the password so the guaranteed characters
    // are not always at the beginning.
    let mut chars: Vec<char> = password.chars().collect();
    chars.shuffle(&mut rng);

    chars.into_iter().collect()
}

fn random_char<R: Rng + ?Sized>(characters: &str, rng: &mut R) -> char {
    let chars: Vec<char> = characters.chars().collect();
    let index = rng.random_range(0..chars.len());

    chars[index]
}

fn main() {
    println!("=====================================");
    println!("       RUST PASSWORD GENERATOR");
    println!("=====================================");

    loop {
        let length = loop {
            print!("\nEnter password length (4-256): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<usize>() {
                Ok(value) if (4..=256).contains(&value) => break value,
                _ => println!("Please enter a number between 4 and 256."),
            }
        };

        println!("\nChoose password difficulty:");
        println!("1. Easy");
        println!("   Lowercase + Numbers");
        println!("2. Medium");
        println!("   Lowercase + Uppercase + Numbers");
        println!("3. Hard");
        println!("   Lowercase + Uppercase + Numbers + Symbols");
        println!("4. Extreme");
        println!("   Strong combination of all character types");

        let difficulty = loop {
            print!("\nEnter difficulty (1-4): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().parse::<u32>() {
                Ok(value) if (1..=4).contains(&value) => break value,
                _ => println!("Please enter a number from 1 to 4."),
            }
        };

        let password = generate_password(length, difficulty);

        println!("\n=====================================");
        println!("Generated Password:");
        println!("\n{}", password);
        println!("=====================================");

        let difficulty_name = match difficulty {
            1 => "Easy",
            2 => "Medium",
            3 => "Hard",
            4 => "Extreme",
            _ => "Unknown",
        };

        println!("Password Length : {}", password.chars().count());
        println!("Difficulty      : {}", difficulty_name);

        print!("\nGenerate another password? (Y/N): ");
        io::stdout().flush().unwrap();

        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();

        if !again.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    println!("\nPassword Generator closed.");
}