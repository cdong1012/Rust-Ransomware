#[cfg(windows)]
extern crate winapi;

use std::ptr::null_mut;
use std::vec::Vec;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::windef::POINT;
use winapi::um::debugapi::IsDebuggerPresent;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameW};
use winapi::um::synchapi::Sleep;
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winuser::{
    GetAsyncKeyState, GetCursorPos, GetLastInputInfo, LASTINPUTINFO, VK_RBUTTON,
};
pub fn check_debugger() -> bool {
    unsafe {
        match IsDebuggerPresent() {
            0 => {
                return false;
            }
            _ => {
                return true;
            }
        }
    }
}

pub fn check_idle_time() -> bool {
    unsafe {
        let mut last_input_info: LASTINPUTINFO = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0u32,
        };
        GetLastInputInfo(&mut last_input_info);
        let idle_time: u32 = (GetTickCount() - last_input_info.dwTime) / 1000;
        if idle_time >= 60 {
            return true;
        }
        return false;
    }
}

pub fn sleep_for_an_hour() {
    unsafe { Sleep(3600000) }
}

pub fn check_mouse_click(min_clicks: u32) {
    let mut count: u32 = 0;

    while count < min_clicks {
        let key_left_clicked = unsafe { GetAsyncKeyState(VK_RBUTTON) };
        if key_left_clicked >> 15 == -1 {
            count += 1;
        }
        unsafe { Sleep(100) };
    }
}

pub fn check_cursor_position() -> bool {
    let mut cursor: POINT = POINT { x: 0i32, y: 0i32 };
    unsafe {
        GetCursorPos(&mut cursor);
        let x = cursor.x;
        let y = cursor.y;
        Sleep(5000);
        GetCursorPos(&mut cursor);

        if x == cursor.x && y == cursor.y {
            return false;
        }
    }

    true
}

pub fn print_process_name_and_id(processID: u32) -> String {
    unsafe {
        let mut process_name: String = String::new();
        let hProcess: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, processID);
        if hProcess != null_mut() {
            let mut hMod: HMODULE = null_mut();
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                hProcess,
                &mut hMod,
                std::mem::size_of::<HMODULE>() as u32,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                return String::new();
            }
            let mut szProcessName: Vec<u16> = Vec::new();
            let mut count = 0;
            while count < 20 {
                szProcessName.push(0u16);
                count += 1;
            }
            GetModuleBaseNameW(hProcess, hMod, szProcessName.as_ptr() as *mut u16, 20);

            count = 0;

            while szProcessName[count as usize] != 0 {
                count += 1;
            }
            process_name = String::from_utf16(&szProcessName[..count as usize]).unwrap();
        }
        CloseHandle(hProcess);
        return process_name;
    }
}

pub fn check_process() -> bool {
    let mut a_processes: Vec<u32> = Vec::with_capacity(1024);
    let mut i = 0;
    while i < 1024 {
        a_processes.push(0u32);
        i += 1;
    }
    let mut cb_needed: u32 = 0u32;
    let mut _c_processes: u32 = 0u32;
    if unsafe { EnumProcesses(a_processes.as_ptr() as *mut u32, 1024 * 4, &mut cb_needed) } == 0 {
        return false;
    }

    // Calculate how many process identifiers were returned.
    _c_processes = cb_needed / 4;
    let mut current_processes: Vec<String> = Vec::new();
    let mut count = 0;
    while count < _c_processes {
        if a_processes[count as usize] != 0 {
            let process_name = print_process_name_and_id(a_processes[count as usize]);
            if process_name.len() != 0 {
                current_processes.push(process_name);
            }
        }
        count += 1;
    }

    let sandbox_processes = [
        "vmsrvc.exe",
        "tcpview.exe",
        "wireshark.exe",
        "fiddler.exe",
        "vmware.exe",
        "VirtualBox.exe",
        "procexp.exe",
        "autoit.exe",
        "vboxtray.exe",
        "vmtoolsd.exe",
        "vmrawdsk.sys.",
        "vmusbmouse.sys.",
        "df5serv.exe",
        "vboxservice.exe",
    ]
    .to_vec();

    let mut found_processes: Vec<&str> = Vec::new();
    for process in current_processes.iter() {
        for sandbox_process in sandbox_processes.iter() {
            if &(process.to_lowercase()) == &(sandbox_process.to_lowercase()) {
                found_processes.push(sandbox_process);
            }
        }
    }

    if found_processes.len() != 0 {
        return false;
    }

    true
}
