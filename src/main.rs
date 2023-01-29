use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your weight on Earth");
    io::stdin().read_line(&mut input).unwrap();
    println!("Your weight on Earth is {}", input);
    let weight = input.trim().parse::<f64>().unwrap();
    let jupyter_weight = calculate_weight_on_jupyter(weight);
    println!("Your weight on Jupiter is {}", jupyter_weight);
}

fn calculate_weight_on_jupyter(weight : f64) -> f64 {
    let jupiter_gravity = 24.79;
    (weight  / 9.87) * jupiter_gravity 
}


