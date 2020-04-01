extern crate task2;

#[test]
fn square_one() {
    assert_eq!(task2::square(1), 1);
}

#[test]
fn square_two() {
    assert_eq!(task2::square(2), 2);
}

#[test]
fn square_three() {
    assert_eq!(task2::square(3), 4);
}

#[test]
fn square_four() {
    assert_eq!(task2::square(4), 8);
}

#[test]
fn square_sixteen() {
    assert_eq!(task2::square(16), 32_768);
}

#[test]
fn square_thirty_two() {
    assert_eq!(task2::square(32), 2_147_483_648);
}

#[test]
fn square_sixty_four() {
    assert_eq!(task2::square(64), 9_223_372_036_854_775_808);
}

#[test]
fn total_sums_all_squares() {
    assert_eq!(task2::total(), 18_446_744_073_709_551_615);
}
