#[cfg(windows)]
extern crate winapi;

mod lib;

fn main() {
    use lib::check_process;
    check_process();
    println!("Fuck");
}
