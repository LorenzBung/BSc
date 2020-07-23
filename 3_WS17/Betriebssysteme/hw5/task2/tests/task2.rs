extern crate task2;

use task2::*;

#[test]
fn test_one() {
    assert_eq!("I", Roman::from(1));
}

#[test]
fn test_two() {
    assert_eq!("II", Roman::from(2));
}

#[test]
fn test_three() {
    assert_eq!("III", Roman::from(3));
}

#[test]
fn test_four() {
    assert_eq!("IV", Roman::from(4));
}

#[test]
fn test_five() {
    assert_eq!("V", Roman::from(5));
}

#[test]
fn test_six() {
    assert_eq!("VI", Roman::from(6));
}

#[test]
fn test_nine() {
    assert_eq!("IX", Roman::from(9));
}

#[test]
fn test_twenty_seven() {
    assert_eq!("XXVII", Roman::from(27));
}

#[test]
fn test_forty_eight() {
    assert_eq!("XLVIII", Roman::from(48));
}

#[test]
fn test_fifty_nine() {
    assert_eq!("LIX", Roman::from(59));
}

#[test]
fn test_ninety_three() {
    assert_eq!("XCIII", Roman::from(93));
}

#[test]
fn test_141() {
    assert_eq!("CXLI", Roman::from(141));
}

#[test]
fn test_163() {
    assert_eq!("CLXIII", Roman::from(163));
}

#[test]
fn test_402() {
    assert_eq!("CDII", Roman::from(402));
}

#[test]
fn test_575() {
    assert_eq!("DLXXV", Roman::from(575));
}

#[test]
fn test_911() {
    assert_eq!("CMXI", Roman::from(911));
}

#[test]
fn test_1024() {
    assert_eq!("MXXIV", Roman::from(1024));
}

#[test]
fn test_3000() {
    assert_eq!("MMM", Roman::from(3000));
}
