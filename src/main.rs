use rand::prelude::*;
use std::io;

fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
    fruit_selected == random_fruit
}

fn main() {
    let guess_list = ["grapes", "apple", "banana", "orange", "licky", "watermelon"];
    let mut random = thread_rng();

    let index = random.gen_range(0..guess_list.len());
    let random_fruit = guess_list[index];

    println!(
        "Please select from: {}, {}, {},{},{},{}",
        guess_list[0],
        guess_list[1],
        guess_list[2],
        guess_list[3],
        guess_list[4],
        guess_list[5]
    );
    let mut input = String::new();
    
    loop {
        input.clear();
        println!("Please guess a fruit:");
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit selected: {}", fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Fruit entered does not exist, please try again.");
                    continue;
                }

                if guess_checker(&fruit_selected, random_fruit) {
                    println!("You are the winner!");
                    break;
                } else {
                    println!("Incorrect guess, please try again.");
                }
            }
            Err(error) => {
                println!("Error: {}", error);
                break;
            }
        }
    }
}
