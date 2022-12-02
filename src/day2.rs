
type Round = (usize, usize);


fn score((me, op): Round) -> i32 {
    let scoring = [1, 2, 0, 0, 1, 2, 2, 0, 2];

    0
}


pub fn handler(input: &String) -> Result<(), String> {
    // parse input
    let rounds = input
        .lines()
        .map(|line| {
            if let Some((me, op)) = line.split_once(" ") {
                if let Some(me) = "ABC".find(me) {
                    if let Some(op) = "XYZ".find(op) {
                        return Ok((me, op));
                    }
                }
            }

            Err("Parse error".to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;

    for round in rounds {
        println!("{:?}", round);
    }

    Ok(())
}