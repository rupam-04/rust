use std::io;

fn main()
{
    let a: i32;
    let b: i32;
    println!("Enter two numbers :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    a = input.trim().parse().expect("Wanted a number");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    b = input.trim().parse().expect("Wanted a number");
    extended_gcd(a, b);
}

// Find GCD of given two given number
fn extended_gcd(a: i32, b: i32)
{
    // Declare some auxiliary variable
    let mut s: i32 = 0;
    let mut r: i32 = b;
    let mut old_s: i32 = 1;
    let mut old_r: i32 = a;
    let mut temp: i32 ;
    let mut bezout_t: i32 = 0;
    // When r not equal to zero
    while r != 0
    {
        let quotient: i32 = old_r / r;
        temp = r;
        r = old_r - quotient * r;
        old_r = temp;
        temp = s;
        s = old_s - quotient * s;
        old_s = temp;
    }
    if b != 0
    {
        bezout_t = (old_r - old_s * a) / b;
    }
    // Display given number
    println!("\nGiven (a,b) : {},{}",  a , b);
    // Displaying the value calculate result
    println!("Bezout coefficients : {},{}",old_s , bezout_t);
    println!("greatest common divisor : {}", old_r);
}