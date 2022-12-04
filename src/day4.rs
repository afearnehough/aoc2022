pub fn handler(input: &String) -> Result<(), String> {
    let pairs = input
        .lines()
        .map(|line| {
            line.split(|char| char == '-' || char == ',')
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    println!(
        "Part A: {}",
        pairs
            .iter()
            .map(|pair| {
                let (lx0, lx1, rx0, rx1) = (pair[0], pair[1], pair[2], pair[3]);
                ((lx0 <= rx0 && lx1 >= rx1) || (rx0 <= lx0 && rx1 >= lx1)) as i32
            })
            .sum::<i32>());

    println!(
        "Part B: {}",
        pairs
            .iter()
            .map(|pair| {
                let (lx0, lx1, rx0, rx1) = (pair[0], pair[1], pair[2], pair[3]);
                ((lx0 <= rx0 && lx1 >= rx0) || (rx0 <= lx0 && rx1 >= lx0)) as i32

            })
            .sum::<i32>());

    Ok(())
}