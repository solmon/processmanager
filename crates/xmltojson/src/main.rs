use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use std::borrow::Cow;
use quick_xml::name::QName;
use std::fs;
use std::result::Result;
use ftp::FtpStream;
use std::str;

#[derive(Debug)]
struct NewsFeed {
    title: String,
    link: String,
    description: String,
    items: Vec<NewsItem>,
}

#[derive(Debug, Clone)]
struct NewsItem {
    title: String,
    link: String,
    description: String,
}   

impl NewsItem {
    fn new() -> NewsItem {
        NewsItem {
            title: String::from(""),
            link: String::from(""),
            description: String::from(""),
        }
    }
    fn news_element(&mut self,
        reader: &mut Reader<&[u8]>,
        element: BytesStart) -> Result<(), quick_xml::Error> {

        loop {         
            let mut main_buf = Vec::new();               
            match reader.read_event_into(&mut main_buf)? {
                Event::Start(element) => {
                    let name = element.name();
                    if name == QName(b"title") {
                        //title = reader.read_text(element.name())?;  
                        self.title = reader.read_text(element.name())?.into();
                    }
                    else if name == QName(b"link") {
                        //link = reader.read_text(element.name())?;
                        self.link = reader.read_text(element.name())?.into();
                    }
                    else if name == QName(b"description") {
                        //description = reader.read_text(element.name())?;
                        self.description = reader.read_text(element.name())?.into();
                    }                    
                    else {}
                }
                Event::End(element) => {
                    if element.name() == QName(b"item") {
                        //println!("End of title");
                        break
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            main_buf.clear();
        }            
        Ok(())
    }
}

impl NewsFeed {
    fn new() -> NewsFeed {
        NewsFeed {
            title: String::from(""),
            link: String::from(""),
            description: String::from(""),
            items: Vec::new(),
        }
    }       
    fn news_element(&mut self, 
        reader: &mut Reader<&[u8]>,
        element: BytesStart) -> Result<(), quick_xml::Error> {

        let mut title = Cow::Borrowed("");
        let mut link = Cow::Borrowed("");
        let mut description = Cow::Borrowed("");

        loop {         
            let mut main_buf = Vec::new();               
            match reader.read_event_into(&mut main_buf)? {
                Event::Start(element) => {
                    let name = element.name();
                    if name == QName(b"title") {
                        title = reader.read_text(element.name())?;
                        self.title = title.into();                                                  
                    }
                    else if name == QName(b"link") {
                        link = reader.read_text(element.name())?;
                        self.link = link.into();
                    }
                    else if name == QName(b"description") {
                        description = reader.read_text(element.name())?;
                        self.description = description.into();
                    }
                    else if name == QName(b"item") {
                        let mut newsitem = NewsItem::new();
                        newsitem.news_element(reader, element); 
                        self.items.push(newsitem);
                    }
                    else {}
                }
                Event::Eof => break,
                _ => {}
            }
            main_buf.clear();
        }
        //format!("<channel><title>{}</title><link>{}</link><description>{}</description>", self.title, self.link, self.description)
        Ok(())
    }
}


fn main() -> Result<(), quick_xml::Error> {
    
    let mut buf = Vec::new();    
    
    //let input: String = fs::read_to_string("../../tests/documents/samplenews.xml")?;
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
    
    let mut reader = Reader::from_str(input);
    reader.config_mut().trim_text(true);
    let mut newsfeed = NewsFeed::new(); 

    loop {
        match reader.read_event_into(&mut buf)?{
            Event::Start(element) => {                
                if let b"channel" = element.name().as_ref() {
                    println!("Channel found");                    
                    newsfeed.news_element(&mut reader, element);                                        
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    let struct_str = format!("{:#?}", newsfeed);
    println!("{}", struct_str);
    
    Ok(())    
}
