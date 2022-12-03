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
        "Part A: {:?}",
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

    // let rucksacks = input
    //         .lines()
    //         .map(|line| {
    //             let (first, second) = line.split_at(line.len() / 2);
    //             let convert = |char: &u8| -> i32 {
    //                 (if char.is_ascii_uppercase() { char - 38 } else { char - 96 }) as i32
    //             };

    //             let first = first.as_bytes().iter().map(convert).collect::<Vec<i32>>();
    //             let second = second.as_bytes().iter().map(convert).collect::<Vec<i32>>();

    //             (first, second)
    //         });

    //     println!(
    //         "Part A: {}",
    //         rucksacks
    //             .map(|(&first, &second)| {
    //                 let first = first.iter().collect::<HashSet<i32>>();
    //                 let second = second.iter().collect::<HashSet<i32>>();
    //                 let intersect = first.intersection(&second).into_iter().collect::<Vec<&i32>>();
    //                 *intersect[0]
    //             })
    //             .collect::<Vec<i32>>()
    //             .iter()
    //             .sum::<i32>());



    Ok(())
}