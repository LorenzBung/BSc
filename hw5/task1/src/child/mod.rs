use nix::unistd::{fork, getpid};
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};


pub fn run_childs(start_pid: i32, arg: &str) -> Result<(), String> {
    let count = arg.parse::<u8>();
    match count {
        Ok(value) => {
            for i in 0..value {
                let pid = fork();
                match pid {
                    Ok(Child) => {
                        println!("hello, I am child (pid:{})", getpid());
                    }
                    Ok(Parent { child }) => {
                        println!("hello, I am parent of {} (pid:{})", child, getpid());
                        match wait(){
                            Ok(ws) => { println!("{:?}", ws); }
                            Err(_) => {}
                        }
                    }
                    // panic, fork should never fail unless there is a
                    // serious problem with the OS
                    Err(_) => panic!("fork failed"),
                }
            }
            Ok(())
        },
        Err(_) => {
            Err("Parse Failed".to_string())
        },
    }
}