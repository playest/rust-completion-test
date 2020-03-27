use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::Error;
use std::fs::{OpenOptions};

#[derive(Clone)]
pub struct Driver { }

impl Driver {
    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        let f = Rc::new(RefCell::new(File::create("a").unwrap()));
        Attribute {
            file: f
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    file: Rc<RefCell<File>>,
}

pub trait Device {
    fn get_attribute(&self, name: &str) -> Attribute;
}

impl Attribute {
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Result<Attribute, Error> {
        let file = OpenOptions::new().open(&"a")?;
        Ok(Attribute {
            file: Rc::new(RefCell::new(file)),
        })
    }

    fn get_str(&self) -> Result<String, Error> {
        Ok("a".to_owned())
    }
}
