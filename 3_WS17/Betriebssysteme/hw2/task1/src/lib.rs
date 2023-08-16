#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize, String> {
    //Check if the given Strings are of different length
    if s1.len() != s2.len() {
        return Err("Strings must be of equal length!".to_string());
    }
    let mut dist: usize = 0;
    for i in 0..s1.len() {
        //Compare each character of the Strings
        if s1.chars().nth(i) != s2.chars().nth(i) {
            //If they don't match, increment hamming distance
            dist += 1
        }
    }
    Ok(dist)
}
