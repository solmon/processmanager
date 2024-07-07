use ftp::FtpStream;
use std::str;

pub fn getmessage(machine_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    
    let mut ftp_stream = FtpStream::connect("127.0.0.1:21").unwrap();
    let _ = ftp_stream.login("one", "1234rieter").unwrap();

    println!("FTP connection established");
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    ftp_stream.cwd("/data").unwrap();
    let remote_file = ftp_stream.simple_retr("samplenews.xml").unwrap();
    //println!("Read file with contents\n{}\n", str::from_utf8(&remote_file.into_inner()).unwrap());
    //println!("Current directory: {}", ftp_stream.pwd().unwrap());
    let binding = remote_file.into_inner();
    let input = str::from_utf8(&binding).unwrap();
    let _ = ftp_stream.quit();

    Ok(input.into())   
}
