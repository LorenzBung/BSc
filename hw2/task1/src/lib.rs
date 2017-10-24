#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, String> {
    if s1.len() != s2.len() {
        return Err("Strings must be of equal length!".to_string());
    }
    let mut dist: usize = 0;
    for i in 0..s1.len() {
        if s1.chars().nth(i) != s2.chars().nth(i) {
            dist += 1
        }
    }
    Ok(dist)
}
