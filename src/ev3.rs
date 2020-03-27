use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Driver {
    class_name: String,
    name: String,
    attributes: RefCell<HashMap<String, Attribute>>,
}

impl Driver {
    pub fn new() -> Driver {
        Driver {
            class_name: "a".to_owned(),
            name: "a".to_owned(),
            attributes: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        let attributes = self.attributes.borrow_mut();
        attributes.get(attribute_name).expect("Internal error in the attribute map").clone()
    }
}

use std::io::Error;


#[derive(Debug, Clone)]
pub struct Attribute {
    file: Rc<RefCell<File>>,
}

pub trait Device {
    fn get_attribute(&self, name: &str) -> Attribute;
}

use std::fs::{OpenOptions};

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
