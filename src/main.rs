use std::{
    fs::File,
    io::Read
};

mod day1;

fn main() {
    let days = [
        (1, day1::run)
    ];

    for (day, handler) in days {
        println!("\n\n======== Day {} ========\n\n", day);

        let mut input = match File::open(format!("input/day{}.txt", day)) {
            Ok(file) => file,
            Err(_) => panic!("Couldn't find input for day {}!", day)
        };

        let mut input_data = String::new();
        match input.read_to_string(&mut input_data) {
            Ok(_) => handler(&input_data),
            Err(_) => panic!("Unable to read input for day {}!", day)
        }
    }
}