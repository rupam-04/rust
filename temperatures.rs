use std::io;

fn main() {
    let mut choice: String = String::new();
    loop {
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if choice == 1 {
            println!("Enter temp in Celsius: ");
            let mut temp: String = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let mut temp: f64 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };
            temp = temp * 1.8 + 32.0;
            println!("Temp in Fahrenheit: {}", temp);
        }
        else if choice == 2 {
            println!("Enter temp in Fahrenheit: ");
            let mut temp: String = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
            let mut temp: f64 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => 0.0,
            };
            temp = (temp - 32.0) / 1.8;
            println!("Temp in Celsius: {}", temp);
        }
        else if choice == 3 {
            println!("Exiting...");
            break;
        }
    }
}