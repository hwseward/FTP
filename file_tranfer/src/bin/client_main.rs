use file_transfer::ftp::*;
use std::fs::File;

fn main() -> std::io::Result<()> {

    let ftp = FTP::new("127.0.0.1:8080", File::open("/home/denard/Pictures/background.png").unwrap());
    ftp.send();
    Ok(())

}
