
fn operate(input: &String, part: char) {
    let mut stacks: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        if line.trim().starts_with("move") {
            let params = line
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .collect::<Vec<_>>();
            if let [num, from, to] = params[..] {
                if part == 'A' {
                    for _ in 0..num.min(stacks[from - 1].len()) {
                        if let Some(crv) = stacks[from - 1].pop() {
                            stacks[to - 1].push(crv);
                        }
                    }
                } else if part == 'B' {
                    let stack_from_len = stacks[from - 1].len();
                    let temp = stacks[from - 1].drain(stack_from_len - num..).collect::<Vec<u8>>();
                    stacks[to - 1].extend(temp);
                }
            }
        } else {
            let bytes = line.as_bytes();
            let mut x = 0;
            while (x + 2) < bytes.len() {
                if bytes[x] == '[' as u8 && bytes[x + 2] == ']' as u8 {
                    let stack_num = x / 4;
                    if stacks.len() < stack_num + 1 {
                        stacks.resize(stack_num + 1, Vec::new());
                    }

                    stacks[stack_num].insert(0, bytes[x + 1]);
                }

                x += 4;
            }
        }
    }

    println!("Part {}: ", part);
    for stack in stacks.iter().filter(|stack| !stack.is_empty()) {
        print!("{}", stack[stack.len() - 1] as char);
    }
    println!();
}

pub fn handler(input: &String) -> Result<(), String> {
    operate(input, 'A');
    operate(input, 'B');

    Ok(())
}