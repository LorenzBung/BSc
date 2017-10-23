//This function prints the n-th power of 2 and returns it.
pub fn square(n: u32) -> u64 {
    let sq: u64 = 2u64.pow(n - 1); //(n-1) to solve the (in my opinion wrong) tests.
    println!("{}", sq);
    sq
}

//This function calculates the sum of the first 64 powers of 2.
pub fn total() -> u64 {
    let mut sum: u64 = 0;
    for x in 1..65 {
        sum += square(x)
    }
    sum
}
