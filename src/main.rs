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

    let age : i32 = age.trim()
        .parse::<i32>()
        .expect("You can't parse it");

    calculate_bmi_result(age, bmi);
}

fn calculate_bmi_result(age: i32, bmi: f32) {
    if age > 18 && age < 69 {
        match bmi {
           x if x <= 18.5 => println!("Underweight"),
           x if x > 18.5 && x <= 25.0 => println!("Normal"),
           x if x > 25.0 && x <= 30.0 => println!("Overweight"),
           x if x > 30.0 => println!("Obese"),
           _ => println!("Unknown")
        };
    } else if age > 69 {
        match bmi {
           x if x <= 22.0 => println!("Underweight"),
           x if x > 22.0 && x <= 28.0 => println!("Normal"),
           x if x > 28.0 && x <= 30.0 => println!("Overweight"),
           x if x > 30.0 => println!("Obese"),
           _ => println!("Unknown")
        };
    } else {
        println!("Could not calculate your BMI")
    }
}
