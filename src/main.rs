//  ----------------------  Fruit Guessing  ----------------------  //

// use rand::prelude::*;
// use std::io;

// fn guess_checker(fruit_selected: &str, random_fruit: &str) -> bool {
//     fruit_selected == random_fruit
// }

// fn main() {
//     let guess_list = ["grapes", "apple", "banana", "orange", "licky", "watermelon"];
//     let mut random = thread_rng();

//     let index = random.gen_range(0..guess_list.len());
//     let random_fruit = guess_list[index];

//     println!(
//         "Please select from: {}, {}, {} ,{} ,{} ,{}",
//         guess_list[0],
//         guess_list[1],
//         guess_list[2],
//         guess_list[3],
//         guess_list[4],
//         guess_list[5]
//     );
//     let mut input = String::new();

//     loop {
//         input.clear();
//         println!("Please guess a fruit:");

//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let fruit_selected = input.trim().to_lowercase();
//                 println!("Fruit selected: {}", fruit_selected);

//                 if !guess_list.contains(&fruit_selected.as_str()) {
//                     println!("Fruit entered does not exist, please try again.");
//                     continue;
//                 }

//                 if guess_checker(&fruit_selected, random_fruit) {
//                     println!("You are the winner!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//                 break;
//             }
//         }
//     }
// }

//  ----------------------  Animal Guessing  ----------------------  //

// use rand::prelude::*;
// use std::io;

// fn guess_checker(selected_animal: &str, random_animal: &str) -> bool{
//     selected_animal == random_animal
// }

// fn main() {
//     let guess_list = ["dog", "cat", "cow", "rat", "crow", "frog", "butterfly"];

//     let random_animal = guess_list[thread_rng().gen_range(0..guess_list.len())];

//     println!(
//         "Please select the animal form this list: {}, {}, {}, {}, {}, {}, {}",
//         guess_list[0],
//         guess_list[1],
//         guess_list[2],
//         guess_list[3],
//         guess_list[4],
//         guess_list[5],
//         guess_list[6]
//     );

//     let mut input = String::new();

//     loop {
//         input.clear();
//         println!("Please guess the animal: ");

//         match io::stdin().read_line(&mut input) {
//             Ok(_)=>{
//                 let selected_animal = input.trim().to_lowercase();
//                 println!("Fruit selected: {}", selected_animal);

//                 if !guess_list.contains(&selected_animal.as_str()){
//                     println!("Fruit entered does not exist, please try again.");
//                     continue;
//                 }

//                 if guess_checker(&selected_animal, random_animal) {
//                     println!("You are the winner!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//            Err(error) => {
//            println!("Error: {}", error);
//             break;
//             }
//         }
//     }
// }

//  ----------------------  Color Guessing  ----------------------  //

// use rand::prelude::*;
// use std::io;

// fn guess_checker(selected_color: &str, random_color: &str) -> bool {
//     selected_color == random_color
// }

// fn main() {
//     let guess_list = ["pink", "yellow", "black", "blue", "purple", "green", "red"];

//     let random_color = guess_list[thread_rng().gen_range(0..guess_list.len())];

//     println!(
//         "Please the select color form this list: {}, {} ,{} ,{} ,{} ,{} , {}",
//         guess_list[0],
//         guess_list[1],
//         guess_list[2],
//         guess_list[3],
//         guess_list[4],
//         guess_list[5],
//         guess_list[6]
//     );

//     let mut input = String::new();

//     loop {
//         input.clear();
//         println!("Please guess the color: ");

//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 let selected_color = input.trim().to_lowercase();
//                 println!("Fruit selected: {}", selected_color);
//                 if !guess_list.contains(&selected_color.as_str()) {
//                     println!("Fruit entered does not exist, please try again.");
//                     continue;
//                 }

//                 if guess_checker(&selected_color, random_color) {
//                     println!("You are the winner!");
//                     break;
//                 } else {
//                     println!("Incorrect guess, please try again.");
//                 }
//             }
//             Err(error) => {
//                 println!("Error: {}", error);
//                 break;
//             }
//         }
//     }
// }

//  ----------------------  Number Guessing  ----------------------  //

use rand::prelude::*;
use std::io;

fn guess_checker(selected_number: i32, random_number: i32) -> bool {
    selected_number == random_number
}

fn main() {
    let guess_list = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let random_num = guess_list[thread_rng().gen_range(0..guess_list.len())];

    println!(
        "Please select a number from the list: {}, {}, {}, {}, {}, {}, {}, {}, {}",
        guess_list[0],
        guess_list[1],
        guess_list[2],
        guess_list[3],
        guess_list[4],
        guess_list[5],
        guess_list[6],
        guess_list[7],
        guess_list[8]
    );

    let mut input = String::new();

    loop {
        input.clear();
        println!("Please guess the number: ");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let selected_number: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input, please enter a number.");
                        continue;
                    }
                };

                if !guess_list.contains(&selected_number) {
                    println!("Number entered does not exist in the list, please try again.");
                    continue;
                }

                if guess_checker(selected_number, random_num) {
                    println!("You are the winner!");
                    break;
                } else {
                    println!("Incorrect guess, please try again.");
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                break;
            }
        }
    }
}
