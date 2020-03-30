#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_wrong_diff() {
        assert_eq!(verify_product(42, 13253224, &String::from("12345")), None)
    }

    #[test]
    fn test_hashing() {
        assert_eq!(
            Sha256::hash("test".as_bytes()).hex(),
            "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }
}
