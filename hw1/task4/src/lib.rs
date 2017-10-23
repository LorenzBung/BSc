pub fn square_of_sum(n: i32) -> i32 {
    let mut i: i32 = 0;
    for x in 1..n+1 {
        i += x;
    }
    i.pow(2)
}

pub fn sum_of_squares(n: i32) -> i32 {
    let mut i: i32 = 0;
    for x in 1..n+1 {
       i += x.pow(2);
    }
    i
}

pub fn difference(n: i32) -> i32 {
    square_of_sum(n) - sum_of_squares(n)
}
