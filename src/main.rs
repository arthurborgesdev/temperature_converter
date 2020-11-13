use std::io;

fn main() {
    // Get temperature metric
    loop {
        println!("Type actual temperature metric ('C' for Celsius, 'F' for Farenheit) - Type 'Q' to quit the program");
    
        let mut temperature_metric = String::new();
    
        io::stdin()
            .read_line(&mut temperature_metric)
            .expect("Failed to read line");
    
        let temperature_metric: char = match temperature_metric.trim().parse() {
            Ok(metric) => metric,
            Err(_) => continue,
        };

        if temperature_metric == 'Q' { break }

        if temperature_metric != 'C' && temperature_metric != 'F' { continue }

        let temperature_value = get_temperature_value();

        if temperature_metric == 'C' {
            let farenheit_value = (temperature_value * 9.0 / 5.0) + 32.0;
            println!("{} Celsius degrees are equal {} Farenheit degrees", temperature_value, farenheit_value);
        } else if temperature_metric == 'F' {
            let celsius_value = (temperature_value - 32.0) * 5.0 / 9.0;
            println!("{} Farenheit degrees are equal {} Celsius degrees", temperature_value, celsius_value);
        } 
    }
        
}

fn get_temperature_value() -> f64 {
    loop {
        println!("Type the value that you want to convert");

        let mut temperature_value = String::new();

        io::stdin()
            .read_line(&mut temperature_value)
            .expect("Failed to read line");

        let temperature_value: f64 = match temperature_value.trim().parse() {
            Ok(metric) => metric,
            Err(_) => continue,
        };
        break temperature_value;
    }
}