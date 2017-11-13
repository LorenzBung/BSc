extern crate libc;

use procinfo::pid;
use self::libc::pid_t;

struct Process {
    name : String,
    pid : pid_t,
    ppid : pid_t,
}

impl Process {

    fn new(with_pid:pid_t) -> Self {
        if let Ok(stat) = pid::stat(with_pid) {
            Process{name: stat.command, pid:stat.pid, ppid:stat.ppid}
        } else {
            panic!("Internal Error: Process not found")
        }
    }

    fn has_parent(&self) -> bool {
        self.ppid != 0
    }

    fn parent(&self) -> Self {
        Process::new(self.ppid)
    }

    fn has_parent_with_pid(&self, pid: pid_t) -> bool {
        if self.pid == pid {
            return true
        }

        if self.has_parent() {
            return self.parent().has_parent_with_pid(pid)
        }

        false
    }

    fn print_recursive(&self, to_pid:pid_t, output: &mut String) {

        if output.len() == 0 {
            *output = format!("{}({}){}", self.name, self.pid, output);
        } else {
            *output = format!("{}({})---{}", self.name, self.pid, output);
        }

        if self.has_parent() && self.pid != to_pid {
            self.parent().print_recursive(to_pid, output);
        }
    }
}

pub fn print(pid:pid_t) -> bool {

    if let Err(_) = pid::stat(pid) {
        println!("Invalid PID");
        return false
    }

    if let Ok(my_pid) = pid::stat_self() {
        let mut output = "".to_string();
        let my_proc = Process::new(my_pid.pid);

        if !my_proc.has_parent_with_pid(pid) {
            println!("This Process has no parent {}", pid);
            return false
        }

        my_proc.print_recursive(pid, &mut output);
        println!("{}", output);
    }

    true
}