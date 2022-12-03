pub fn handler(input: &String) -> Result<(), String> {
    let rucksacks = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|char: &u8| -> i32 {
                    (if char.is_ascii_uppercase() { char - 38 } else { char - 96 }) as i32
                })
                .collect::<Vec<i32>>()
        }).collect::<Vec<Vec<i32>>>();

    println!(
        "Part A: {}",
        rucksacks
            .iter()
            .map(|rucksack| {
                let compartments = rucksack.chunks(rucksack.len() / 2).collect::<Vec<&[i32]>>();
                for item in compartments[0] {
                    if compartments[1].contains(item) {
                        return *item;
                    }
                }

                0
            })
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>());

    println!(
        "Part B: {:?}",
        rucksacks
            .chunks(3)
            .map(|group| {
                for item in &group[0] {
                    if group[1].contains(item) && group[2].contains(item) {
                        return *item;
                    }
                }

                0
            })
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>());

    Ok(())
}