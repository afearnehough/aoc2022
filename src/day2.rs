type Round = (usize, usize);

fn score((op, me): Round) -> i32 {
    let scoring = [
        1, 2, 0,
        0, 1, 2,
        2, 0, 1
    ];

    (me as i32 + 1) + scoring[me + op * 3] * 3
}

pub fn handler(input: &String) -> Result<(), String> {
    let rounds = input
        .lines()
        .map(|line| {
            if let Some((op, me)) = line.split_once(" ") {
                if let Some(op) = "ABC".find(op) {
                    if let Some(me) = "XYZ".find(me) {
                        return Ok((op, me));
                    }
                }
            }

            Err("Parse error".to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;


    println!(
        "Part A: Total score: {}",
        rounds
            .iter()
            .fold(0, |sum, round| sum + score(*round)));

    println!(
        "Part B: Total score: {}",
        rounds
            .iter()
            .fold(0, |sum, round| {
                let (op, me) = *round;
                let which = [
                    2, 0, 1,
                    0, 1, 2,
                    1, 2, 0
                ];

                sum + score((op, which[me + op * 3]))
            }));

    Ok(())
}