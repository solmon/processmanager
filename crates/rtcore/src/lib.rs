use std::error::Error;

pub trait RMessageReader {
    fn read(&self) -> Result<&str, Box<dyn Error>>;
}