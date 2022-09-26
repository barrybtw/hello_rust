use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=3);
    println!("The secret number is: {}", secret_number);

    let computer_choice = if secret_number == 1 {
        "rock"
    } else if secret_number == 2 {
        "paper"
    } else {
        "scissors"
    };

    println!("The computer chose: {}", computer_choice);

    println!("What is your choice? (rock, paper, scissors)");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    let user_choice = user_choice.trim();

    println!("You chose: {}", user_choice);

    if user_choice == computer_choice {
        println!("You tied!");
    } else if user_choice == "rock" && computer_choice == "scissors" {
        println!("You won!");
    } else if user_choice == "paper" && computer_choice == "rock" {
        println!("You won!");
    } else if user_choice == "scissors" && computer_choice == "paper" {
        println!("You won!");
    } else {
        println!("You lost!");
    }

    println!("Thanks for playing!");
    // don't close the window
    let mut close = String::new();
    io::stdin().read_line(&mut close).expect("Failed to read line");
}