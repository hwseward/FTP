use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};

pub enum FTP {

    
    // Normal(String::From("127.0.0.1:8080"), String::From("/home/user/file"))
    Normal(String, File),
    // Secure(String::From("127.0.0.1:8080"), String::From("/home/user/file"))
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
    /// let ftp = FTP::new("127.0.0.1:8080", "");
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
    /// let ftp = FTP::new("127.0.0.1:8080", "/home/user/file.txt");
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

    pub fn recv(self) -> String {

        //Creates a Vector to hold the files
        let mut files = Vec::new();
        //Creates buffer to read stream
        let mut buffer = Vec::new();
        // Gets ip from self
        let (ip, _) = self.get_ip_file();
        // Starts Listening from incoming clients
        let listener = TcpListener::bind(ip).unwrap();
        // handles each client
        for stream in listener.incoming() {

            // sets num to the size of the file
            let num = stream.unwrap().read_to_end(&mut buffer).unwrap();
            let test = String::from_utf8(buffer[..num].to_vec()).unwrap()
            test
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
