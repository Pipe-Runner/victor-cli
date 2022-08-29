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

    println!("Please select an option:\n1. Add two vectors\n2. Subtract two vectors\n3. Multiply a scalar to a vector\n4. Angle between two vectors\n5. Dot product of two vectors\n6. Cross product of two 3D vectors\n7. Generate 3D basis from one vector\n8. Generate 3D basis from two vectors\n9. Exit");
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
            println!("\nThe result is:\n{}", result);
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
            println!("\nThe result is:\n{}", result);
        }
        3 => {
            let mut input: String;
            println!("Please enter a first vector: (single space separated numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter a scalar:");
            input = read_line();
            let k = input.parse::<f32>().unwrap();

            let result = v1.scalar_mul(k);
            println!("\nThe result is:\n{}", result);
        }
        4 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter the second vector: (single space separated numbers)");
            input = read_line();
            let v2 = convert_string_to_vector(&input);

            let result = v1.find_angle_between(&v2);
            println!("\nThe result is:\n{} rad(s)", result);
        }
        5 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter the second vector: (single space separated numbers)");
            input = read_line();
            let v2 = convert_string_to_vector(&input);

            let result = v1.dot(&v2);
            println!("\nThe result is:\n{}", result);
        }
        6 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated three numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter the second vector: (single space separated three numbers)");
            input = read_line();
            let v2 = convert_string_to_vector(&input);

            let result = v1.cross(&v2);
            println!("\nThe result is:\n{}", result);
        }
        7 => {
            let input: String;
            println!("Please enter a vector: (single space separated three numbers)");
            input = read_line();
            let v = convert_string_to_vector(&input);

            let result = Vector::find_basis_with_one(&v);
            println!("\nThe result is:\n{}\n{}\n{}", result.0, result.1, result.2);
        }
        8 => {
            let mut input: String;
            println!("Please enter the first vector: (single space separated three numbers)");
            input = read_line();
            let v1 = convert_string_to_vector(&input);

            println!("Please enter the second vector: (single space separated three numbers)");
            input = read_line();
            let v2 = convert_string_to_vector(&input);

            let result = Vector::find_basis_with_two(&v1, &v2);
            println!("\nThe result is:\n{}\n{}\n{}", result.0, result.1, result.2);
        }
        _ => {
            println!("Goodbye");
            exit(0);
        }
    }
}
