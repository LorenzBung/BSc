extern crate procinfo;

use std::env;
use std::process;

mod readproc;
mod pstree;

mod unit_test_pstree;
mod unit_test_readproc;

/// Mainfunction
fn main() {

    let args:Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            if let Ok(pid_tuple) = readproc::self_pids() {
                let pid = pid_tuple.0;
                let ppid = pid_tuple.1;

                // Commands & Threads for PID
                if let Ok(pid_command) = readproc::get_pid_command(pid){
                    if let Ok(pid_threads) = readproc::get_thread_count(pid){
                        println!("My PID : {} - {} running {} threads", pid, pid_command, pid_threads);
                    }
                }

                // Commands & Threads for Parent-PID
                if let Ok(ppid_command) = readproc::get_pid_command(ppid){
                    if let Ok(ppid_threads) = readproc::get_thread_count(ppid){
                        println!("My PPID: {} - {} running {} threads", ppid, ppid_command, ppid_threads);
                    }
                }

            }


            if let Ok(size_tuple) = readproc::get_ownprocess_mem() {
                // Memory
                let vspace = size_tuple.0;
                let code = size_tuple.1;
                let data = size_tuple.2;

                println!("My mem : {} (vspace), {} (code), {} (data)", vspace, code, data);
            }

            if let Ok(last_command) = readproc::get_last_created_command() {
                // Last Process
                println!("Last process created in system was: {}", last_command);
            }


            if let Ok(task_total) = readproc::get_task_total() {
                // Number of tasks
                println!("Total number of tasks: {}", task_total);
            }
        }

        2 => {
            match args[1].parse::<i32>() {
                Ok(pid) => {
                   if !pstree::print(pid) {
                       process::exit(1);
                   }
                }
                Err(_) => {
                    println!("Error while parsing PID");
                    process::exit(1);
                }
            }
        }

        _ => {
            println!("Correct usage: no param or param PID");
            process::exit(1);
        }
    }
}
