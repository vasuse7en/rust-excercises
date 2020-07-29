use std::io;

fn from_celsius_to_fahrenheit(temp: u32) -> u32 {
    return temp * 9 / 5 + 32;
}

fn from_fahrenheit_to_elsius(temp: u32) -> u32 {
    return (temp - 32) * 5 / 9;
}

pub fn convert_temperatures() {
    println!("Please choose the action you want.");
    println!("1. Convert from Celsius to Fahrenheit");
    println!("2. Convert from Fahrenheit to Celsius");

    // ask user for action
    let mut option_chosen = String::new();
    loop {
        io::stdin()
            .read_line(&mut option_chosen)
            .expect("Failed to read line");

        let option_chosen: u32 = option_chosen.trim().parse().expect("Please type a number!");

        if ![1, 2].contains(&option_chosen) {
            println!("Please choose either 1 or 2");
            continue;
        }

        break;
    }
    let option_chosen: u32 = option_chosen.trim().parse().expect("Please type a number!");

    // ask user for value to convert
    let mut input_value = String::new();
    if option_chosen == 1 {
        println!("Enter Celcius value to be onverted to Fahrenheit");
    } else {
        println!("Enter Fahrenheit value to be converted to Celcius");
    }

    loop {
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        let _input_value: u32 = input_value.trim().parse().expect("Please type a number!");

        break;
    }

    let input_value: u32 = input_value.trim().parse().expect("Please type a number!");
    if option_chosen == 1 {
        let result: u32 = from_celsius_to_fahrenheit(input_value);
        println!("In Fahrenheit {}", result);
    } else {
        let result: u32 = from_fahrenheit_to_elsius(input_value);
        println!("In Celsius {}", result);
    }
}
