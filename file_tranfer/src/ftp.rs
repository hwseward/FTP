use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::fs::File;


pub enum FTP {

    
    // Normal(String::From("127.0.0.1:8080"), String::From("/home/user/file"))
    // Secure(String::From("127.0.0.1:8080"), String::From("/home/user/file"))
    Normal(String, File),
    Secure(String, File),

}

// Implantation of Struct FTP
impl FTP {

    /* Returns and FTP with your ip and port
     *
     * Parameters &str with ip and port and str with path to file
     *
     * Example
     * ```
     * let ftp = FTP::new("127.0.0.1:8080", "");
     * ```
     */

    pub fn new(ip_addr: &str, file: File) -> FTP {

        // Converts &str to String
        let ip_addr = String::from(ip_addr);
        // returns new FTP
        FTP::Normal(ip_addr, file)

    }

    pub fn send(self) {

        let (ip, file) = self.get_ip_file();
        let stream = 
    
    }

    /* Returns the file you gave when declaring the FTP obj
     *
     * Example
     * ```
     * let file: File = ftp.get_file();
     * ```
     */

     fn get_ip_file(self) -> (String, File) {
    
        // Checks what type of FTP it is
        match self {
   
            FTP::Normal(ip, file) =>  return (ip, file),
            FTP::Secure(ip, file) =>  return (ip, file)

       } 

    }

}
