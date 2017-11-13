#[cfg(test)]
mod tests {
    use pstree::Process;
    use pstree::print;

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
        assert_eq!(true, Process::has_parent(&Process::me()));
    }

    #[test]
    #[should_panic]
    fn parent_not_existing() {
        Process::parent(&Process::new(1));
    }

    #[test]
    fn parent_existing() {
        Process::parent(&Process::me());
    }

    #[test]
    fn hasparent_with_pid_not_existing() {
        assert_eq!(false, Process::has_parent_with_pid(&Process::me(), -1000));
    }

    #[test]
    fn hasparent_with_pid_existing() {
        assert_eq!(true, Process::me().has_parent_with_pid(1));
    }

    #[test]
    fn print_not_existing() {
        assert_eq!(false, print(-1000));
    }

    #[test]
    fn print_existing() {
        assert_eq!(true, print(1));
    }

    #[test]
    fn print_recursive_existing_pid() {
        let prc = Process::new(1);
        let mut str = String::new();
        prc.print_recursive(1, &mut str);
        assert_eq!("systemd(1)", str);
    }
}