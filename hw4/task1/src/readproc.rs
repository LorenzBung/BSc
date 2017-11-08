use procinfo::pid;
use procinfo::loadavg;

pub fn self_pids() -> Result<(i32, i32), &'static str>{
    match pid::stat_self() {
        Ok(stat) => { Ok((stat.pid, stat.ppid)) }
        Err(_) => { Err("PID not alive: PID and PPID not found") }
    }
}

pub fn get_pid_command(pid: i32) -> Result<String, &'static str> {
    match pid::stat(pid) {
        Ok(stat) => { Ok(stat.command) }
        Err(_) => { Err("PID not alive: no command name found") }
    }
}

/*pub fn get_last_created_command() -> Result<String, &'static str> {
    match loadavg() {
        Ok(stat) => {
            let last_pid = stat.last_created_pid;
            match pid::stat(last_pid) {
                Ok(st) => { Ok(st.command) }
                Err(_) => { Err("No last command via PID found") }
            }
        }
        Err(_) => { Err("No last command via PID found") }
    }
}*/

pub fn get_thread_count(pid: i32) -> Result<u32, &'static str> {
    match pid::stat(pid) {
        Ok(stat) => { Ok(stat.num_threads as u32) }
        Err(_) => { Err("PID not alive: no threads counted") }
    }
}

pub fn get_task_total() -> Result<u32, &'static str> {
    match loadavg() {
        Ok(stat) => { Ok(stat.tasks_total) }
        Err(_) => { Err("No total count of tasks in system found") }
    }
}

pub fn get_ownprocess_mem() -> Result<(usize, usize, usize), &'static str> {
    match pid::stat_self() {
        Ok(stat) => {
            let csize = stat.end_code - stat.start_code;
            let dsize = stat.end_data - stat.start_data;
            Ok((stat.vsize, csize, dsize)) }
        Err(_) => { Err("PID not alive: no memory found") }
    }
}
