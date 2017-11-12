#[cfg(test)]
mod tests {
    use procinfo::pid::{status, status_self};
    use {get_ownprocess_mem, get_pid_command, get_task_total, get_thread_count, self_pids};


    fn sol_self_pids() -> (i32, i32) {
        match status_self() {
            Ok(status) => (status.pid, status.ppid),
            Err(_) => panic!(),
        }
    }

    fn name_of_init() -> String {
        status(1).unwrap().command
    }

    #[test]
    fn test_name_of_init() {
        let status = status(1).unwrap();
        assert_eq!(name_of_init(), status.command);
    }

    #[test]
    fn test0_ppid() {
        assert_eq!(sol_self_pids(), self_pids().unwrap());
    }

    #[test]
    fn test1_command() {
        assert_eq!(
            Err("PID not alive: no command name found"),
            get_pid_command(0)
        );
    }

    #[test]
    fn test2_command() {
        assert_eq!(Ok(name_of_init()), get_pid_command(1));
    }


    #[test]
    fn test3_systemd_threads() {
        let status = status(1).unwrap();
        assert_eq!(get_thread_count(1), Ok(status.threads));
    }

    // Only check if fn is defined

    #[test]
    #[should_panic]
    fn test8_mem() {
        assert_eq!(Ok((0, 0, 0)), get_ownprocess_mem());
    }

    #[test]
    #[should_panic]
    fn test9_get_task_total() {
        assert_eq!(Ok((0)), get_task_total());
    }

}
