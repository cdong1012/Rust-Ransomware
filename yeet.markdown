
# Rust Ransomware: Part 1
# Setting up & Anti Rerverse Engineering technique in malwares

### 1. Set up
- To set up this lab, please make sure you have a recent version of [Rust](https://www.rust-lang.org/tools/install "Rust Installation") installed.
- Create a folder on your computer and change into that directory from your Command Prompt

```
    cd folder
    cargo init
```

- You should see a few files and folders created like below
    ![alt text](https://github.com/cdong1012/Rust-Ransomware/blob/master/image/Cargoinit.JPG "Cargo init")

    ⋅⋅* The **src** folder is where you should put your Rust codes in for the malware.
    ⋅⋅* The **target** folder is where you can find the products of your code after building it (The .exe file for the malware,...)
    ⋅⋅* *Cargo.toml* is a file where you can specify the dependencies that your code might need (it's similar to *import* in python)

- Before starting, append this to your *Cargo.toml* file. Inside the **features** array, we can include the crates that we use from [Rust-Winapi](https://docs.rs/winapi/0.3.8/winapi/index.html "Rust Winapi"). For example, if I want to use Winduser.h on Windows, I can import it as below

```

    [target.'cfg(windows)'.dependencies]
    winapi = { version = "0.3", features = ["winuser"] }

```
        
### 2. Anti Reverse Engineering techniques
1. **IsDebuggerPresent**
    - IsDebuggerPresent is a cool WinAPI function used to check for the BeingDebugged flag in the PEB (Process Environment Block) 
        and will return a non-zero value if it is indeed being debug.
    
    - In theory, if this functions returns a non-zero value, the malware should exit immediately instead of executing its behavior to prevent reverse engineers from being able to run it with a debugger attached

    - You can read more about the documentation [here](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent "IsDebuggerPresent")
    
    - This is what the documentation from Rust Winapi looks like
        ![alt text](https://github.com/cdong1012/Rust-Ransomware/blob/master/image/IsDebuggerPresent.JPG "IsDebuggerPresent")

    - If you trace down the type of the returned variable (*BOOL*), you will find that *BOOL* is just a wrapper for *i32* in Rust!
    - At this point, we're ready to try it out in main.rs!

    - First, since *IsDebuggerPresent* is from the winapi::um::debugapi crate, we need to import it in **Cargo.toml**.

    ```
        winapi = { version = "0.3", features = ["debugapi"}

    ```

    - After that, we can lay it out in **main.rs**:

    ``` Rust

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


    ```

    - First, we check if IsDebuggerPresent() returns a 0 or any other number. If it's 0, the program is not being debugged, so we continue to print "Hello, world!"
    - If it's being debugged, we print the debug code out and call std::process::exit(0) to exit immediately!
    - Here is the result
        1. Double clicking on the executable /target/debug/Rust-Ransomware.exe. As you can see, the program prints out "Hello, world!"

        ![alt text](https://github.com/cdong1012/Rust-Ransomware/blob/master/image/noDebugger.JPG "No debugger")

        1. Debugging this executable in IDA, we can set a break point where we compare eax(the return value from IsDebuggerPresent()). If we execute to this point, you can see that eax = 1, so we will exit immediately!

        ![alt text](https://github.com/cdong1012/Rust-Ransomware/blob/master/image/debuggerIDA.JPG "Debugger")

        ![alt text](https://github.com/cdong1012/Rust-Ransomware/blob/master/image/debuggerIDA2.JPG "Debugger")

