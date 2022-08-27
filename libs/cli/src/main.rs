use std::{io, process::exit};
use vector_processor::Vector;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn convert_string_to_vector(input: &str) -> Vector {
    let mut numbers: Vec<f32> = Vec::new();
    for number in input.split_whitespace() {
        numbers.push(number.trim().parse::<f32>().unwrap());
    }
    Vector::new(numbers)
}

fn main() {
    println!("Welcome to Victor: Not so friendly Vector Processor\n");

    println!("Please select an option:\n 1. Add two vectors\n 2. Subtract two vectors\n 3. Multiply a scalar to a vector\n 5. Exit");
    let operation: i32 = read_line().trim().parse::<i32>().unwrap();

    match operation {
        1 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter the second vector: (single space separated numbers)");
            input = read_line();
            let v2 = convert_string_to_vector(&input);

            let result = v1.add(v2);
            println!("\nThe result is:\n {}", result);
        }
        2 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter the second vector: (single space separated numbers)");
            input = read_line();
            let v2 = convert_string_to_vector(&input);

            let result = v1.sub(v2);
            println!("\nThe result is:\n {}", result);
        }
        3 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter a scalar:");
            input = read_line();
            let k = input.parse::<f32>().unwrap();

            let result = v1.scalar_mul(k);
            println!("\nThe result is:\n {}", result);
        }
        _ => {
            println!("Goodbye");
            exit(0);
        }
    }
}
