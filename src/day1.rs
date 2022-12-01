pub fn handler(input: &String) {
    let mut elves = vec![0];
    for line in input.trim().split("\n").map(|line| line.trim()) {
        if line.is_empty() {
            elves.push(0);
        } else if let Some(elf) = elves.last_mut() {
            if let Ok(value) = line.parse::<i32>() {
                *elf += value;
            }
        }
    }

    elves.sort();

    println!("Highest calorie count: {}", elves[elves.len() - 1]);
    println!("Top 3 calories sum: {}", elves[elves.len()-3..].iter().sum::<i32>());
}