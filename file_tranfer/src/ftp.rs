use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};
use std::io::prelude::*;

pub enum FTP {

    
    // Normal(String::From("127.0.0.1:8080"), File::create("/home/user/file"))
    Normal(String, File),
    // Secure(String::From("127.0.0.1:8080"), File::create("/home/user/file"))
    Secure(String, File),

}

// Implantation of Struct FTP
impl FTP {

    /// Returns and FTP with your ip and port
    ///
    /// Parameters &str with ip and port and str with path to file
    ///
    /// Example
    /// ```
    /// let ftp = FTP::new("127.0.0.1:8080", File::create("/home/user/file.txt"));
    /// ```

    pub fn new(ip_addr: &str, file: File) -> FTP {

        // Converts &str to String
        let ip_addr = String::from(ip_addr);
        // returns new FTP
        FTP::Normal(ip_addr, file)

    }

    
    /// Sends file to desiered Ip
    /// Parameter self
    ///
    /// Example
    /// ```
    /// let ftp = FTP::new("127.0.0.1:8080", File::create("/home/user/file.txt"));
    /// ftp.send();
    /// ```

    pub fn send(self) {

        // Creates buffer to read file
        let mut buffer = Vec::new();
        // Gets Ip and file from self
        let (ip, mut file) = self.get_ip_file();
        // Connects to desired IP
        let mut stream = TcpStream::connect(ip).unwrap();
        // reads file to buffer
        file.read_to_end(&mut buffer).unwrap();
        stream.write_all(&buffer).unwrap();

    }

    /// Receives file and writes it to the desiered opened file
    /// 
    /// Parameter self
    ///
    /// Example
    /// ```
    /// let ftp = FTP::new("127.0.0.1:8080", File::create("/home/user/file.txt"));
    /// ftp.send();
    /// ```

    pub fn recv(self) {

        //Creates buffer to read stream
        let mut buffer = Vec::new();
        // Gets ip from self
        let (ip, mut file) = self.get_ip_file();
        // Starts Listening from incoming clients
        let listener = TcpListener::bind(ip).unwrap();
        // handles each client
        for stream in listener.incoming() {

            // sets num to the size of the file
            let num = stream.unwrap().read_to_end(&mut buffer).unwrap();
            // Writes file data to opened file
            file.write_all(&buffer[..num]).unwrap();
        }
        

    }
    
    /// Returns the file you gave when declaring the FTP obj
    ///
    /// Example
    /// ```
    /// let file: File = ftp.get_ip_file();
    /// ```
    
    fn get_ip_file(self) -> (String, File) {
    
        // Checks what type of FTP it is
        match self {
   
            FTP::Normal(ip, file) =>  return (ip, file),
            FTP::Secure(ip, file) =>  return (ip, file)

       } 

    }

}
