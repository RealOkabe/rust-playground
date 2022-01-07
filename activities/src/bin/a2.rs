// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn display_result(arg: i32) {
    println!("{:?}", arg);
}

fn main() {
    let sum = add_numbers(8, 3);
    display_result(sum);
}
