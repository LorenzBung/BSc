#[cfg(test)]
mod test {
    use nix::unistd::{pipe, write};
    use {concatenate_strings, mul_strings, read_from_pipe, split_into_strings, sum_strings};

    #[test]
    fn test_pipe() {
        match pipe() {
            Ok((reader, writer)) => {
                let input = b"teststring";

                if let Err(_) = write(writer, input) {
                    // Broken pipe
                }
                assert_eq!(read_from_pipe(reader).unwrap(), "teststring".to_string());
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_mul_strings() {
        let input = vec!["32".to_string(), "12".to_string(), "-2".to_string()];
        assert_eq!(mul_strings(input).unwrap(), -768);
    }

    #[test]
    fn test_sum_strings() {
        let input = vec!["6542".to_string(), "542".to_string(), "192".to_string()];
        assert_eq!(sum_strings(input).unwrap(), 7276);
    }

    #[test]
    fn test_split_strings() {
        assert_eq!(
            split_into_strings("Das ist ein Test"),
            vec!["Das", "ist", "ein", "Test"]
        );
    }

    #[test]
    fn test_split_strings_nums() {
        assert_eq!(
            split_into_strings("1 -2 -4 -2 51232"),
            vec!["1", "-2", "-4", "-2", "51232"]
        );
    }

    #[test]
    fn test_concat_string() {
        assert_eq!(concatenate_strings("Hello", "World"), "HelloWorld");
    }
}
