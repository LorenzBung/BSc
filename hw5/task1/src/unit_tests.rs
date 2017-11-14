#[cfg(test)]
mod tests {
    use child::*;

    fn test_zero_forks(){
        assert_eq!(run_childs(123,"2"))
    }
}