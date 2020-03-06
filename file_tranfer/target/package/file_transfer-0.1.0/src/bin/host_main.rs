use file_transfer::ftp::*;
use std::fs::File;


/// Example
fn main() -> std::io::Result<()> {

    // Creates new FTP
    let ftp = FTP::new("127.0.0.1:8080", File::create("/home/denard/Pictures/Test.png").unwrap());
    // Receives File
    ftp.recv();
    Ok(())

}
