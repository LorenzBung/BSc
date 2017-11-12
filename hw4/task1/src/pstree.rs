use procinfo::pid;


pub fn print(pid:i32) -> bool{
    if let Err(_) = pid::stat(pid) {
        return false
    }

    if let Ok(my_pid) = pid::stat_self() {
        let mut output = "".to_string();
        print_recursive(my_pid.pid, pid, &mut output);
        println!("{}", output);
    }

    true
}

fn print_recursive(from_pid:i32, to_pid:i32, output: &mut String) {
    if let Ok(stat) = pid::stat(from_pid) {

        if output.len() == 0 {
            *output = format!("{}({}){}", stat.command, stat.pid, output);
        } else {
            *output = format!("{}({})---{}", stat.command, stat.pid, output);
        }


        if stat.ppid != 0 && stat.pid != to_pid {
            print_recursive(stat.ppid, to_pid, output);
        }
    }
}


