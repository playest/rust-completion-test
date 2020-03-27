use std::io::Error;

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
    fn get_str(&self) -> Result<String, Error> {
        Ok("a".to_owned())
    }
}
