extern crate task4;

#[test]
fn test_square_of_sum_5() {
    assert_eq!(225, task4::square_of_sum(5));
}

#[test]
fn test_sum_of_squares_5() {
    assert_eq!(55, task4::sum_of_squares(5));
}

#[test]
fn test_difference_5() {
    assert_eq!(170, task4::difference(5));
}

#[test]
fn test_square_of_sum_100() {
    assert_eq!(25502500, task4::square_of_sum(100));
}

#[test]
fn test_sum_of_squares_100() {
    assert_eq!(338350, task4::sum_of_squares(100));
}

#[test]
fn test_difference_100() {
    assert_eq!(25164150, task4::difference(100));
}
