#[cfg(test)]
mod tests {
    use hash256;

    #[test]
    fn test_verify1() {
        assert_eq!(
            hash256::verify_product(42, 567621, "12345"),
            Some(hash256::Solution(
                567621,
                "b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345"
                    .to_string(),
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_verify2() {
        assert_eq!(
            hash256::verify_product(41, 567621, "12345"),
            Some(hash256::Solution(
                567621,
                "b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345"
                    .to_string(),
            ))
        );
    }

    #[test]
    #[should_panic]
    fn test_verify3() {
        assert_eq!(
            hash256::verify_product(42, 567620, "12345"),
            Some(hash256::Solution(
                567621,
                "b6bea2a40ed1bd6d9999b2232072092f3df0e02c4b507aa3ad947367b9712345"
                    .to_string(),
            ))
        );
    }

    #[test]
    fn test_verify4() {
        assert_eq!(hash256::verify_product(41, 567621, "12345"), None);
    }

    #[test]
    fn test_verify5() {
        assert_eq!(hash256::verify_product(42, 567622, "12345"), None);
    }
}
