use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn game(you: char, computer: char) -> i32 {
    if you == computer { return -1; }

    if you == 's' && computer == 'p' {
        return 0;
    } else if you == 'p' && computer == 's' {
        return 1;
    }

    if you == 's' && computer == 'z' {
        return 1;
    } else if you == 'z' && computer == 's' {
        return 0;
    }

    if you == 'p' && computer == 'z' {
        return 0;
    } else if you == 'z' && computer == 'p' {
        return 1;
    }

    -1
}

fn main() {
    let n: u64;
    let mut you = String::new();
    let computer: char;
    let result: i32;

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs();

    n = seed % 100;

    if n < 33 {
        computer = 's';
    } else if n > 33 && n < 66 {
        computer = 'p';
    } else {
        computer = 'z';
    }

    println!("\n\n\n\n\t\t\t\tEnter s for STONE, p for PAPER and z for SCISSOR\n\t\t\t\t\t\t");

    io::stdin().read_line(&mut you).expect("Failed to read line");
    let you = you.trim().chars().next().expect("Empty input");

    result = game(you, computer);

    if result == -1 {
        println!("\n\n\t\t\t\tGame Draw!\n");
    } else if result == 1 {
        println!("\n\n\t\t\t\tWow! You have won the game!\n");
    } else {
        println!("\n\n\t\t\t\tOh! You have lost the game!\n");
    }

    println!("\t\t\t\tYou choose : {} and Computer choose : {} \n", you, computer);
}
