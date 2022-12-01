pub fn handler(input: &String) {
    let mut elves = Vec::new();
    for line in input.trim().split("\n").map(|line| line.trim()) {
        if line.is_empty() {
            elves.push(0);
        } else if let Some(elf) = elves.last_mut() {
            if let Ok(value) = line.parse::<i32>() {
                *elf += value;
            }
        }
    }

    elves.sort_by(|elf1, elf2| elf1.cmp(&elf2));

    println!("Highest calorie count: {}", elves[elves.len() - 1]);
    println!("Top 3 calories sum: {}", elves[elves.len()-3..].iter().sum::<i32>());
}