use std::io;

fn main() {
    let mut player_hp = 100;
    let mut ufo_hp = 75;
    let mut input = String::new();

    println!("Welcome to UFO RPG!");

    loop {
        println!("Your HP: {}", player_hp);
        println!("UFO HP: {}", ufo_hp);
        println!("What would you like to do? (attack/heal)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() == "attack" {
            let damage = rand::random::<u8>() % 10 + 1;
            println!("You deal {} damage to the UFO!", damage);
            ufo_hp -= damage;

            if ufo_hp <= 0 {
                println!("You defeated the UFO!");
                break;
            }

            let damage = rand::random::<u8>() % 10 + 1;
            println!("The UFO attacks you for {} damage!", damage);
            player_hp -= damage;

            if player_hp <= 0 {
                println!("You have been defeated by the UFO!");
                break;
            }
        } else if input.trim() == "heal" {
            let heal_amount = rand::random::<u8>() % 10 + 1;
            println!("You heal yourself for {} HP!", heal_amount);
            player_hp += heal_amount;

            let damage = rand::random::<u8>() % 10 + 1;
            println!("The UFO attacks you for {} damage!", damage);
            player_hp -= damage;

            if player_hp <= 0 {
                println!("You have been defeated by the UFO!");
                break;
            }
        } else {
            println!("Invalid input! Please try again.");
        }

        input.clear();
    }
}
