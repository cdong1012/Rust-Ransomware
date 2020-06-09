#[cfg(windows)]
extern crate winapi;

use winapi::um::debugapi::IsDebuggerPresent;

fn main() {
    unsafe {
        match IsDebuggerPresent() {
            0 => {
                println!("Debugger is not present... Continue");
            },
            _ => {
                println!("Debugger is present... Terminating. Code {}", IsDebuggerPresent());
                std::process::exit(0);
            }
        }
    }

    println!("Hello, world!");
    loop {}
}
