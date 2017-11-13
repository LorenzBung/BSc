use procinfo::pid;
use std::os::unix::raw::pid_t;

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
            Process{name: "".to_string(), pid:0, ppid:0}
        }
    }

    fn has_parent(&self) -> bool {
        self.ppid != 0
    }

    fn parent(&self) -> Self {
        Process::new(self.ppid)
    }

    fn has_parent_with_pid(&self, pid: pid_t) -> bool {
        let mut p = self;

        if p.pid == pid {
            return true
        }

        if p.has_parent() {
            return p.parent().has_parent_with_pid(pid)
        }
        false
    }

    fn print_recursive(d, to_pid:pid_t, output: &mut String) {

        if output.len() == 0 {
            *output = format!("{}({}){}", from.name, from.pid, output);
        } else {
            *output = format!("{}({})---{}", from.name, from.pid, output);
        }

        if from.has_parent() && from.pid != to_pid {
            Process::print_recursive(&from.parent(), to_pid, output);
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

        Process::print_recursive(&my_proc,pid, &mut output);
        println!("{}", output);
    }

    true
}