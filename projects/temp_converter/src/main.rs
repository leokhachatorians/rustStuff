use std::io::{self, BufRead};
use std::ascii::AsciiExt;

fn main() {
    let choice = get_conversion_type();
    let temprature = get_temp();
    convert(choice, temprature);
}

fn get_conversion_type()-> String {
	/*
	Get the user input choice of either celsius or fahrenheit and
	return the input lowercase.
	*/
	println!("Do you want to convert to Celsius or Fahrenheit?");

	let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let input = match lines.next() {
        Some(Ok(a)) => a,
        _ => panic!("Couldn't read input.")
    };
    input.to_ascii_lowercase()
}

fn get_temp() -> f64 {
	let mut temprature = String::new();
	println!("Please enter the temprature");

	io::stdin().read_line(&mut temprature)
		.ok()
		.expect("Error reading");
	
	let temprature: f64 = match temprature.trim().parse() {
		Ok(num) => num,
		Err(_) => get_temp(),
	};
	temprature
}

fn convert(choice: String, temprature: f64){
	if choice == "fahrenheit"{
		let fahr = temprature * (9.0/5.0) + 32.0;
		println!("That is {:.2} degrees Fahrenheit", fahr);
	}
	else if choice == "celsius"{
		let cel = (5.0/9.0) * (temprature - 32.0);
		println!("That is {:.2} degrees Celsius", cel);
	}
	else{
		println!("You didn't specify 'Celsius' or 'Fahrenheit'");
		main();
	}
}