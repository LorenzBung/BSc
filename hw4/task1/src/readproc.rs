use procinfo::pid;
use procinfo::loadavg;

/// Returns the PID and PPID of the current process.
/// Throws an error if the current process doesn't exist (should never occur).
pub fn self_pids() -> Result<(i32, i32), &'static str> {
    match pid::stat_self() {
        Ok(stat) => Ok((stat.pid, stat.ppid)),
        Err(_) => Err("PID not alive: PID and PPID not found"),
    }
}

/// Returns the command (string) belonging to the given PID.
/// Throws an error if the given PID doesn't exist.
pub fn get_pid_command(pid: i32) -> Result<String, &'static str> {
    match pid::stat(pid) {
        Ok(stat) => Ok(stat.command),
        Err(_) => Err("PID not alive: no command name found"),
    }
}

/// Returns the last created command (string) of the system.
/// Throws an error if there is no last Command.
pub fn get_last_created_command() -> Result<String, &'static str> {
    match loadavg() {
        Ok(stat) => {
            let last_pid = stat.last_created_pid;
            match pid::stat(last_pid) {
                Ok(st) => Ok(st.command),
                Err(_) => Err("No last command via PID found"),
            }
        }
        Err(_) => Err("No last command found"),
    }
}

/// Returns the number of threads belonging to the given PID.
/// Throws an error if the given PID doesn't exist.
pub fn get_thread_count(pid: i32) -> Result<u32, &'static str> {
    match pid::stat(pid) {
        Ok(stat) => Ok(stat.num_threads as u32),
        Err(_) => Err("PID not alive: no threads counted"),
    }
}

/// Returns the number of total tasks running in the system.
/// Throws an error if the total number of tasks doesn't exist.
pub fn get_task_total() -> Result<u32, &'static str> {
    match loadavg() {
        Ok(stat) => Ok(stat.tasks_total),
        Err(_) => Err("No total count of tasks in system found"),
    }
}

/// Returns the size of the virtual, code and data memory size of the current process.
/// Throws an error if the current process doesn't exist (should never occur).
pub fn get_ownprocess_mem() -> Result<(usize, usize, usize), &'static str> {
    match pid::stat_self() {
        Ok(stat) => {
            let csize = stat.end_code - stat.start_code;
            let dsize = stat.end_data - stat.start_data;
            Ok((stat.vsize, csize, dsize))
        }
        Err(_) => Err("PID not alive: no memory found"),
    }
}
