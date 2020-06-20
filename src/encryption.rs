#[cfg(windows)]
extern crate winapi;

use std::ffi::CString;
use std::ptr::null_mut;
use winapi::shared::winerror::NTE_NO_KEY;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDecrypt, CryptDestroyKey, CryptEncrypt, CryptExportKey, CryptGenKey,
    CryptGetUserKey, CryptImportKey, CryptReleaseContext, AT_KEYEXCHANGE, CALG_AES_192,
    CRYPT_EXPORTABLE, CRYPT_VERIFYCONTEXT, HCRYPTKEY, HCRYPTPROV, PLAINTEXTKEYBLOB, PROV_RSA_AES,
    SIMPLEBLOB,
};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE,
};

static blob_buffer: [u8; 36] = [
    8, 2, 0, 0, 15, 102, 0, 0, 24, 0, 0, 0, 8, 68, 217, 142, 222, 209, 85, 216, 44, 88, 2, 170,
    248, 210, 84, 119, 53, 196, 64, 96, 252, 205, 231, 229,
];

pub fn encrypt(source_file: CString, dest_file: CString) -> bool {
    let mut h_key: HCRYPTKEY = 0usize; // key
    let mut h_crypt_prov: HCRYPTPROV = 0usize;
    let KEYLENGTH = 0x00C00000; // upper 16 bits = 192 bits

    unsafe {
        // acquiring a cryptographic provider
        if CryptAcquireContextA(
            &mut h_crypt_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            println!(
                "Error during CryptAcquireContext! Errror code: {}",
                GetLastError()
            );
            return false;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        // import AES key
        if CryptImportKey(
            h_crypt_prov,
            blob_buffer.as_ptr(),
            blob_buffer.len() as u32,
            0,
            0,
            &mut h_key,
        ) == 0
        {
            println!("Import fail {:?}", GetLastError());
            return false;
        } else {
            println!("Import sucessful. Key is 0x{:x}", h_key);
        }

        //---------------------------------------------------------------
        // Determine the number of bytes to encrypt at a time.
        // This must be a multiple of 192 since we're doing AES-192.
        let mut block_len: u32 = 960;
        let mut buffer_len: u32 = 960;

        //---------------------------------------------------------------
        // Allocate memory.

        let mut pb_buffer: Vec<u8> = Vec::new();
        pb_buffer.resize(buffer_len as usize, 0u8);
        println!("Memory has been allocated for the buffer.");

        let source_handle: HANDLE = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut dest_handle: HANDLE = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut EOF = 0;
        let mut count = 0;

        while EOF == 0 {
            if ReadFile(
                source_handle,
                pb_buffer.as_ptr() as *mut _,
                block_len,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Error reading");
                break;
            }
            if count < block_len {
                EOF = 1;
            }

            if CryptEncrypt(
                h_key,
                0,
                EOF,
                0,
                pb_buffer.as_ptr() as *mut u8,
                &mut count,
                buffer_len,
            ) == 0
            {
                println!("Fail to encrypt 0x{:x}", GetLastError());
                break;
            }

            if WriteFile(
                dest_handle,
                pb_buffer.as_ptr() as *const _,
                count,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Fail to write");
                break;
            }
        }
        CloseHandle(source_handle);
        CloseHandle(dest_handle);
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
    }
    true
}

pub fn decrypt(source_file: CString, dest_file: CString) -> bool {
    let mut h_key: HCRYPTKEY = 0usize; // key
    let mut h_crypt_prov: HCRYPTPROV = 0usize;
    let KEYLENGTH = 0x00C00000; // upper 16 bits = 192 bits

    unsafe {
        if CryptAcquireContextA(
            &mut h_crypt_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            println!("Error during CryptAcquireContext!");
            println!("Errror code: {}", GetLastError());
            return false;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        if CryptImportKey(
            h_crypt_prov,
            blob_buffer.as_ptr(),
            blob_buffer.len() as u32,
            0,
            0,
            &mut h_key,
        ) == 0
        {
            println!("Import fail {:?}", GetLastError());
            return false;
        } else {
            println!("Import sucessful. Key is {}", h_key);
        }

        let src_handle: HANDLE = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut dest_handle: HANDLE = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut block_len: u32 = 960;
        let mut buffer_len: u32 = 960;

        let mut EOF = 0;
        let mut count = 0;

        let mut pb_buffer: Vec<u8> = Vec::new();
        pb_buffer.resize(buffer_len as usize, 0u8);

        while EOF == 0 {
            if ReadFile(
                src_handle,
                pb_buffer.as_ptr() as *mut _,
                block_len,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Error reading 0x{:x}", GetLastError());
                break;
            }
            println!("count {}", count);
            if count < block_len {
                EOF = 1;
            }

            if CryptDecrypt(h_key, 0, EOF, 0, pb_buffer.as_mut_ptr(), &mut count) == 0 {
                println!("Fail to decrypt 0x{:x}", GetLastError());
                break;
            }

            if WriteFile(
                dest_handle,
                pb_buffer.as_ptr() as *const _,
                count,
                &mut count,
                null_mut(),
            ) == 0
            {
                println!("Fail to write");
                break;
            }
        }
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
        CloseHandle(src_handle);
        CloseHandle(dest_handle);
    }
    true
}

pub fn generate_key() -> Vec<u8> {
    let mut h_key: HCRYPTKEY = 0usize; // key
    let mut h_crypt_prov: HCRYPTPROV = 0usize;
    let KEYLENGTH = 0x00C00000; // upper 16 bits = 192 bits

    let mut blob_buff: Vec<u8> = Vec::new();

    unsafe {
        if CryptAcquireContextA(
            &mut h_crypt_prov,
            null_mut(),
            null_mut(),
            PROV_RSA_AES,
            CRYPT_VERIFYCONTEXT,
        ) == 0
        {
            println!(
                "Error during CryptAcquireContext! Errror code: {}",
                GetLastError()
            );
            return blob_buff;
        } else {
            println!("A cryptographic provider has been acquired.");
        }

        if CryptGenKey(
            h_crypt_prov,                 // hProv, handle to key container
            CALG_AES_192,                 // Algid, algorithm ID
            KEYLENGTH | CRYPT_EXPORTABLE, // dwFlags, specifies the type of key generated
            &mut h_key,                   // phKey, mutable pointer to our key
        ) == 0
        {
            println!("Error while generating key");
            return blob_buff;
        } else {
            println!("Finish generating key. Our key is 0x{:x}", h_key);
        }

        let mut blob_length: u32 = 0;
        if CryptExportKey(h_key, 0, PLAINTEXTKEYBLOB, 0, null_mut(), &mut blob_length) == 0 {
            println!("Error while finding blob_length");
            return blob_buff;
        } else {
            println!("Blob buffer has the length of {} bytes", blob_length);
        }

        blob_buff.resize(blob_length as usize, 0u8);
        if CryptExportKey(
            h_key,
            0,
            PLAINTEXTKEYBLOB,
            0,
            blob_buff.as_mut_ptr(),
            &mut blob_length,
        ) == 0
        {
            println!("Error while exporting key");
            return blob_buff;
        } else {
            println!("Done exporting");
        }
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
    }
    blob_buff
}
