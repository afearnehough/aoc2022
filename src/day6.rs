fn first_marker_after(data: &[u8], count: usize) -> Option<usize> {
    let mut start = 0;

    for (n, byte) in data.iter().enumerate() {
        if let Some(at) = data[start..n].iter().position(|&b| b == *byte) {
            start += at + 1;
        }

        if n - start == count {
            return Some(n);
        }
    }

    None
}

pub fn handler(input: &String) -> Result<(), String> {
    let data = input.as_bytes();

    println!("Part A: {}", first_marker_after(data, 4).unwrap());
    println!("Part B: {}", first_marker_after(data, 14).unwrap());

    Ok(())
}