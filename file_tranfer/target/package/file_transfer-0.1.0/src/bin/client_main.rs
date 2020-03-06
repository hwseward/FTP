use file_transfer::ftp::*;
use std::fs::File;


/// Example
fn main() -> std::io::Result<()> {

    // Makes a FTP Enum
    let ftp = FTP::new("127.0.0.1:8080", File::open("/home/denard/Pictures/background.png").unwrap());
    // Sends the File
    ftp.send();
    Ok(())

}
