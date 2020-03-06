## Features
- Send and Receive Files over TCP

## Installation

- use as cargo dependency
```
[dependencies]
file_transfer = "0.1.0"
```

## Usage

- host_main.rs
```
use file_transfer::FTP;
use std:fs::File;

fn main() {

    // creating new FTP obj with the paramaters of ip:port and the file that you want to write from received file
    let ftp = FTP::new("127.0.0.1:8080", File::create("example1.txt"));
    // Receives file from client
    ftp.recv();

}
```
- client_main.rs
```
use file_transfer::FTP;
use std::fs::File;

fn main() {

    // Creates new FTP object with paramaters of ip and the file you want to send
    let ftp = FTP::new("127.0.0.1:8080", File::open("example.txt"));
    // Sends File
    ftp.send();

}
```
