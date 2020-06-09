
# Rust Ransomware: Part 1
# Setting up & Anti Rerverse Engineering technique in malwares

1. Set up
    - To set up this lab, please make sure you have a recent version of [Rust](https://www.rust-lang.org/tools/install "Rust Installation") installed.
    - Create a folder on your computer and change into that directory from your Command Prompt

    ```
        cd folder
        cargo init

    ```

    - You should see a few files and folders created like below
        ![alt text](https://ibb.co/1Tvj5zc "Cargo init")

        ⋅⋅* The **src** folder is where you should put your Rust codes in for the malware.
        ⋅⋅* The **target** folder is where you can find the products of your code after building it (The .exe file for the malware,...)
        ⋅⋅* *Cargo.toml* is a file where you can specify the dependencies that your code might need (it's similar to *import* in python)

    - Before starting, append this to your *Cargo.toml* file. Inside the **features** array, we can include the crates that we use from [Rust-Winapi](https://docs.rs/winapi/0.3.8/winapi/index.html "Rust Winapi"). For example, if I want to use Winduser.h on Windows, I can import it as below

    ```

        [target.'cfg(windows)'.dependencies]
        winapi = { version = "0.3", features = ["winuser"] }

    ```
        
2. IsDebuggerPresent
    - IsDebuggerPresent is a cool WinAPI function used to check for the BeingDebugged flag in the PEB (Process Environment Block) 
        and will return a non-zero value if it is indeed being debug.
    - In theory, if this functions returns a non-zero value, the malware should exit immediately instead of executing its behavior to prevent reverse engineers from 
    being able to run it with a debugger attached

    - 