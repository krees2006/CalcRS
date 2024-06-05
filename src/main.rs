use std::{i128, io};

fn main() {
    loop {
        println!("What arthimtic do you want to use? 1. Add, 2. Sub, 3. Div 4. Multi, 5. Quit");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // turn the string into an int
        let choice: i8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, must be a number!");
                return;
            }
        };

        // make sure user can only enter number from 1-5
        if choice < 1 || choice > 5 {
            println!("Number must be 1-5!");
            return;
        }

        match choice {
            1 => add(),
            2 => sub(),
            3 => div(),
            4 => multi(),
            5 => break,
            _ => unreachable!(),
        }
    }
}

fn add() {
    println!("Enter your first number:");
    let mut input1: String = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    println!("Enter your second number:");
    let mut input2: String = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // turn the strings into integers
    let num1: i128 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    let num2: i128 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    println!("Your sum is {}", num1 + num2);
}

fn sub() {
    println!("Enter your first number:");
    let mut input1: String = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    println!("Enter your second number:");
    let mut input2: String = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // turn the strings into integers
    let num1: i128 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    let num2: i128 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    println!("Your sum is {}", num1 - num2);
}

fn div() {
    println!("Enter your first number:");
    let mut input1: String = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    println!("Enter your second number:");
    let mut input2: String = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // turn the strings into integers
    let num1: i128 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    let num2: i128 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    println!("Your sum is {}", num1 / num2);
}

fn multi() {
    println!("Enter your first number:");
    let mut input1: String = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");

    println!("Enter your second number:");
    let mut input2: String = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    // turn the strings into integers
    let num1: i128 = match input1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    let num2: i128 = match input2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, must be a number!");
            return;
        }
    };

    println!("Your sum is {}", num1 * num2);
}
