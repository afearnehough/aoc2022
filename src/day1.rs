pub fn handler(input: &String) -> Result<(), String> {
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

    println!("Part A: Highest calorie count: {}", elves[elves.len() - 1]);
    println!("Part B: Top 3 calories sum: {}", elves[elves.len()-3..].iter().sum::<i32>());

    Ok(())
}