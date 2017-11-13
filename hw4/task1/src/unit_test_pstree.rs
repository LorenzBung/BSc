#[cfg(test)]
mod tests {
    use pstree::Process;

    #[test]
    #[should_panic]
    fn new_invalid_pid() {
        Process::new(-1000);
    }

    #[test]
    fn new_valid_pid() {
        Process::new(1);
    }

    #[test]
    fn hasparent_no() {
        assert_eq!(false, Process::has_parent(&Process::new(1)));
    }

    #[test]
    fn hasparent_yes() {

    }

    #[test]
    #[should_panic]
    fn parent_not_existing() {
        Process::parent(&Process::new(1));
    }
}