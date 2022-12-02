use std::{
    env,
    fs::File,
    io::Read
};

mod day1;
mod day2;

type Day = (i32, fn(&String) -> Result<(), String>);

fn run(days: &[Day]) {
    for (day, handler) in days {
        println!("\n======== Day {} ========\n", day);

        let mut input_file = match File::open(format!("input/day{}.txt", day)) {
            Ok(file) => file,
            Err(_) => panic!("Couldn't find input for day {}!", day)
        };

        let mut input_data = String::new();
        match input_file.read_to_string(&mut input_data) {
            Ok(_) => {
                match handler(&input_data) {
                    Ok(_) => {},
                    Err(err) => println!("Error! {}", err)
                }
            }
            Err(_) => panic!("Unable to read input for day {}!", day)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<Day> = vec![
        (1, day1::handler),
        (2, day2::handler)
    ];

    println!();
    println!("░█████╗░░█████╗░░█████╗░██████╗░░█████╗░██████╗░██████╗░");
    println!("██╔══██╗██╔══██╗██╔══██╗╚════██╗██╔══██╗╚════██╗╚════██╗");
    println!("███████║██║░░██║██║░░╚═╝░░███╔═╝██║░░██║░░███╔═╝░░███╔═╝");
    println!("██╔══██║██║░░██║██║░░██╗██╔══╝░░██║░░██║██╔══╝░░██╔══╝░░");
    println!("██║░░██║╚█████╔╝╚█████╔╝███████╗╚█████╔╝███████╗███████╗");
    println!("╚═╝░░╚═╝░╚════╝░░╚════╝░╚══════╝░╚════╝░╚══════╝╚══════╝");
    println!("Merry Christmas!");

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