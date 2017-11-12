extern crate procinfo;

use std::env;
use std::process;

mod readproc;
mod pstree;

fn main() {

    let args:Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            match readproc::self_pids() {
                Ok(pid_tuple) => {
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
                }
                Err(_) => {}
            }


            match readproc::get_ownprocess_mem() {
                Ok(size_tuple) => {
                    // Memory
                    let vspace = size_tuple.0;
                    let code = size_tuple.1;
                    let data = size_tuple.2;

                    println!("My mem : {} (vspace), {} (code), {} (data)", vspace, code, data);
                }
                Err(_) => {}
            }

            match readproc::get_last_created_command() {
                Ok(last_command) => {
                    // Last Process
                    println!("Last process created in system was: {}", last_command);
                }
                Err(_) => {}
            }


            match readproc::get_task_total() {
                Ok(task_total) => {
                    // Number of tasks
                    println!("Total number of tasks: {}", task_total);
                }
                Err(_) => {}
            }
        }

        2 => {
            match args[1].parse::<i32>() {
                Ok(pid) => {
                   if !pstree::print(pid) {
                       println!("Invalid PID");
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
