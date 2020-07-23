extern crate task1;

#[test]
fn test_vanilla_leap_year() {
    assert_eq!(task1::is_leap_year(1996), true);
}

#[test]
fn test_any_old_year() {
    assert_eq!(task1::is_leap_year(1995), false);
    assert_eq!(task1::is_leap_year(1997), false);
    assert_eq!(task1::is_leap_year(1998), false);
    assert_eq!(task1::is_leap_year(1999), false);
}

#[test]
fn test_century() {
    assert_eq!(task1::is_leap_year(1900), false);
}

#[test]
fn test_exceptional_centuries() {
    assert_eq!(task1::is_leap_year(2000), true);
    assert_eq!(task1::is_leap_year(2400), true);
}
