extern crate procinfo;

mod readproc;
mod pstree;

fn main() {
    println!("Hello, world!");
    println!("{:?}", readproc::self_pids());
}
