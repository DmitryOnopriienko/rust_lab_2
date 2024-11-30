use std::io::stdin;

const WRONG_NUM_INPUT_MSG: &str =
    "Wrong input, expecting a floating point number, for example: 3.14. Try again.";
const WRONG_SYMB_INPUT_MSG: &str =
    "Wrong input, expecting an operation symbol, such as +, -, /, * or %. Try again.";

fn main() {
    println!("Calculator");
    let mut memo: f64 = 0.0;
    loop {
        println!("Enter the first value or M to use memorized value:");
        let n1: f64 = number_input(memo);

        println!("Enter the operation to execute:");
        let operation: String = operation_input();

        println!("Enter the second value or M to use memorized value:");
        let n2: f64 = number_input(memo);

        let result = compute(n1, n2, &operation);

        println!("Result: {} {} {} = {}", n1, operation, n2, result);

        println!("Enter 'E' if you want to exit and 'M' if you want to memorise the result. In other case - enter any other symbol");
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        match user_input.trim() {
            "M" => {
                memo = result;
            }
            "E" => return,
            _ => continue,
        };
    }
}

fn number_input(memo: f64) -> f64 {
    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let trimmed = user_input.trim();
        if trimmed == "M" {
            println!("{}", memo);
            return memo;
        }
        match trimmed.parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!("{}", WRONG_NUM_INPUT_MSG),
        };
    }
}

fn operation_input() -> String {
    loop {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        let trimmed = user_input.trim();
        if ["+", "-", "*", "/", "%"].contains(&trimmed) {
            return trimmed.to_string();
        } else {
            println!("{}", WRONG_SYMB_INPUT_MSG);
        }
    }
}

fn compute(n1: f64, n2: f64, op: &str) -> f64 {
    match op {
        "+" => return n1 + n2,
        "-" => return n1 - n2,
        "*" => return n1 * n2,
        "/" => return n1 / n2,
        "%" => return n1 % n2,
        _ => panic!("Wrong operation passed to compute method, I'm in panic!!!"),
    }
}
