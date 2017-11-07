use procinfo::pid::Stat;
use procinfo::pid;

pub fn self_pids() -> Result<(i32, i32), &'static str>{
    match pid::stat_self() {
        Ok(stat) => { Ok((stat.pid, stat.ppid)) }
        Err(_) => { Err("Fehler") }
    }
}

pub fn get_pid_command(pid: i32) -> Result<String, &'static str> {
    match pid::stat(pid) {
        Ok(stat) => { Ok(stat.command) }
        Err(_) => { Err("Fehler") }
    }
}

pub fn get_last_created_command() -> Result<String, &'static str> {
    match pid::stat_self() {
        Ok(stat) => { Ok(stat.command) }
        Err(_) => { Err("Fehler") }
    }

}

pub fn get_thread_count(pid: i32) -> Result<u32, &'static str> {
    match pid::stat(pid) {
        Ok(stat) => { Ok(stat.num_threads as u32) }
        Err(_) => { Err("Fehler") }
    }
}
