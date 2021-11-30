use std::io::{self, BufRead, Write};

fn main() {
    println!("Welcome to the tip calculator!");
    print!("Please enter the total cost before tip: ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut stdin_line_iterator = stdin.lock().lines();
    let input_cost = stdin_line_iterator.next().unwrap().unwrap();
    let input_cost_as_float: f32 = only_float_chars(&input_cost).parse().unwrap();
    print!("Please enter the tip percentage: ");
    io::stdout().flush().unwrap();
    let input_percentage = stdin_line_iterator.next().unwrap().unwrap();
    let input_percentage_as_float: f32 = only_float_chars(&input_percentage).parse().unwrap();
    println!();
    println!(
        "Tip ({:.0}%): {:.2}",
        &input_percentage_as_float,
        &input_cost_as_float * (&input_percentage_as_float / 100f32)
    );
}

fn only_float_chars(input: &String) -> String {
    let mut return_string = String::from("");
    for current_char in input.chars() {
        if current_char.is_digit(10) || current_char == '.' {
            return_string.push(current_char)
        }
    }
    return return_string;
}
