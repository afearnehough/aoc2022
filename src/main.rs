use std::{
    env,
    fs::File,
    io::Read
};

mod day1;

type Day = (i32, fn(&String));

fn run(days: &[Day]) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<Day> = vec![
        (1, day1::handler)
    ];
    
    if args.len() > 1 {
        if let Ok(value) = args[1].parse::<usize>() {
            if value > 0 && value <= days.len() {
                run(&[days[value - 1]]);
            }
        }
    } else {
        run(&days[..]);
    }
}