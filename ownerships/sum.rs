fn sum(mut x: &i32) {
    x = x + 1;
}

fn main() {
    let mut x = 5;
    sum(&x);
    println!("x is {}", x);
}