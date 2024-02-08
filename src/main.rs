use std::io;

fn main() {
    println!("What is your age?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read line");

    println!("What is your height in meters?");
    let mut height_in_meters = String::new();
    io::stdin().read_line(&mut height_in_meters).expect("Failed to read line");

    println!("What is your weight?");
    let mut weight_in_kilos = String::new();
    io::stdin().read_line(&mut weight_in_kilos).expect("Failed to read line");

    let height_in_meters : f32 = height_in_meters.trim()
        .parse::<f32>()
        .expect("You can't parse it");

    let weight_in_kilos : f32 = weight_in_kilos.trim()
        .parse::<f32>()
        .expect("You can't parse it");

    let bmi = weight_in_kilos / (height_in_meters * height_in_meters);
    println!("Your BMI is: {}", bmi);

    calculate_bmi_result(bmi);
}

fn calculate_bmi_result(bmi: f32) {
    match bmi {
       0.0..=18.5 => println!("Underweight"),
       18.5..=25.0 => println!("Normal"),
       25.0..=30.0 => println!("Overweight"),
       30.0.. => println!("Obese"),
       _ => println!("Unknown")
    };
}
