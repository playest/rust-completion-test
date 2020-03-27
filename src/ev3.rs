use std::io::Error;
use std::fs::{OpenOptions};

#[derive(Clone)]
pub struct Driver { }

impl Driver {
    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        Attribute { }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
}

pub trait Device {
    fn get_attribute(&self, name: &str) -> Attribute;
}

impl Attribute {
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Result<Attribute, Error> {
        let file = OpenOptions::new().open(&"a")?;
        Ok(Attribute { })
    }

    fn get_str(&self) -> Result<String, Error> {
        Ok("a".to_owned())
    }
}
