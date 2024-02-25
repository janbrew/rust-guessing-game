use std::{io, process::exit};

fn check_answer(current_int: i32, answer: String) -> bool {
    if current_int % 3 == 0 && current_int % 5 == 0 {
        answer.as_str().eq("fizzbuzz") 
    }

    else if current_int % 3 == 0 {
        answer.as_str().eq("fizz")
    }
    
    else if current_int % 5 == 0 {
        answer.as_str().eq("buzz")
    }

    else {
        match answer.parse::<i32>() {
            Ok(int_answer) => int_answer == current_int,
            Err(_) => false
        }
    }
}   

fn main() {
    
    let mut current_int: i32 = 1;

    while current_int <= 100 {
        println!("The current number is: {} [fizz, buzz, fizzbuzz]\n*enter number if none*", current_int);
        
        let stdin = io::stdin();
        let mut answer = String::new();

        match stdin.read_line(&mut answer) {
            Ok(_input) => {
                let correct: bool = check_answer(current_int, answer.to_string().trim().to_lowercase());
        
                if !correct {
                    println!("\nWrong answer!\n");
                    continue;
                }
        
                current_int += 1;
            },
            Err(_) => exit(1)
        }
    }
}
