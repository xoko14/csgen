use std::io::{self, BufRead};

pub fn readlines() -> String{
    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input.len() == 0 {
            break;
        }

        if user_input.len() > 0 {
            user_input.push_str("\n");
        }
        
        user_input.push_str(&last_input);
    }

    user_input
}