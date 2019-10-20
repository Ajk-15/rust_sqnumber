use std::io;

 fn main() {
    println!("Enter a number:");
    let mut num_sq = String::new();

    io::stdin().read_line(&mut num_sq).expect("Failed to read line");
    let number: i32 = num_sq.trim().parse().unwrap();

let number = number * number;
println!("The square of the number is {}", number);

}