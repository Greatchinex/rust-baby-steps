use std::io;

fn main() {
    println!("Enter your weight (Kg): ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let weight: f32 = user_input.trim().parse().unwrap();

    println!("USER INPUT ===> {}", user_input);


    let mars_weigt = calculate_weight_on_mars(weight);
    println!("Weight On Mars is {}kg", mars_weigt);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
