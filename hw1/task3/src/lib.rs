pub fn count(line: &str, c: char) -> u64 {
    let mut occurances = 0;
    for char in line.chars() {
        if char == c {
            occurances += 1;
        }
    }
    occurances
}
