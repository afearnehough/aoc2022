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

    // for round in rounds {
    //     println!("{:?} = {}", round, score(round));
    // }

    println!("Total score: {}", rounds
        .iter()
        .fold(0, |sum, round| sum + score(*round)));

    Ok(())
}