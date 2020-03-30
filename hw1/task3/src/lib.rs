pub fn count(line: &str, c: char) -> u64 {
    let mut occurrences = 0;
    for char in line.chars() {
        if char == c {
            occurrences += 1;
        }
    }
    occurrences
}
