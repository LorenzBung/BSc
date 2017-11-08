extern crate procinfo;

mod readproc;
mod pstree;

fn main() {

    // PID and PPID
    let pid_tuple = readproc::self_pids().unwrap();
    let pid = pid_tuple.0;
    let ppid = pid_tuple.1;

    // Commands
    let pid_command = readproc::get_pid_command(pid).unwrap();
    let ppid_command = readproc::get_pid_command(ppid).unwrap();

    // Threads
    let pid_threads = readproc::get_thread_count(pid).unwrap();
    let ppid_threads = readproc::get_thread_count(ppid).unwrap();

    // Output for PID and PPID Information
    println!("My PID : {} - {} running {} threads", pid, pid_command, pid_threads);
    println!("My PPID: {} - {} running {} threads", ppid, ppid_command, ppid_threads);

    // Memory
    let vspace = readproc::get_ownprocess_mem().unwrap().0;
    let code = readproc::get_ownprocess_mem().unwrap().1;
    let data = readproc::get_ownprocess_mem().unwrap().2;
    println!("My mem : {} (vspace), {} (code), {} (data)", vspace, code, data);

    // Last Process
    println!("Last process created in system was: {}", readproc::get_last_created_command().unwrap());

    // Number of tasks
    println!("Total number of tasks: {}", readproc::get_task_total().unwrap());
}
