use std::io;

// ( °F − 32) × 5/9 =  °C
fn main() {

    loop{

        println!("Input temperature in Farenheit");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("🤬🤬🤬 not a number 🤬🤬🤬");
                println!("⏭ try again");
                continue
            }
        };

        let result = convert(temperature);
        println!("result in Farenheit is {}", result)
    }
}


fn convert(degree: f32) -> f32 {
    (degree - 32.0) * (5.0/9.0)
}