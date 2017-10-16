extern crate task3;

#[test]
fn test_one_char() {
    assert_eq!(task3::count("♥ The quick brown fox jumps over the lazy dog. ♥", 'T'),
               1);
}

#[test]
fn test_two_char() {
    assert_eq!(task3::count("♥ The quick brown fox jumps over the lazy dog. ♥",
                     '♥'),
               2);
}

#[test]
#[should_panic]
fn test_wrong() {
    assert_eq!(task3::count("♥ The quick brown fox jumps over the lazy dog. ♥", 'c'),
               2);
}

#[test]
fn test_four_char() {
    assert_eq!(task3::count("♥ The quick brown fox jumps over the lazy dog. ♥", 'o'),
               4);
}

#[test]
fn test_no_char() {
    assert_eq!(task3::count("♥ The quick brown fox jumps over the lazy dog. ♥", '!'),
               0);
}
