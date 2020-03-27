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

#[derive(Debug)]
pub enum Ev3Error {
    InternalError {
        msg: String,
    },
}

pub type Ev3Result<T> = Result<T, Ev3Error>;

impl From<std::io::Error> for Ev3Error {
    fn from(err: std::io::Error) -> Self {
        Ev3Error::InternalError {
            msg: format!("{}", err),
        }
    }
}

impl From<std::string::FromUtf8Error> for Ev3Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Ev3Error::InternalError {
            msg: format!("{}", err),
        }
    }
}

impl From<std::num::ParseIntError> for Ev3Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Ev3Error::InternalError {
            msg: format!("{}", err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    file: Rc<RefCell<File>>,
}

pub trait Device {
    fn get_attribute(&self, name: &str) -> Attribute;

    fn get_address(&self) -> Ev3Result<String> {
        self.get_attribute("address").get()
    }

    fn set_command(&self, command: &str) -> Ev3Result<()> {
        self.get_attribute("command").set_str_slice(command)
    }

    fn get_commands(&self) -> Ev3Result<Vec<String>> {
        self.get_attribute("commands").get_vec()
    }

    fn get_driver_name(&self) -> Ev3Result<String> {
        self.get_attribute("driver_name").get()
    }
}

use std::os::unix::io::RawFd;
use std::error::Error;
use std::fs::{OpenOptions};
use std::os::unix::io::AsRawFd;

impl Attribute {
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Ev3Result<Attribute> {
        let file = OpenOptions::new().open(&"a")?;
        Ok(Attribute {
            file: Rc::new(RefCell::new(file)),
        })
    }

    fn get_str(&self) -> Ev3Result<String> {
        Ok("a".to_owned())
    }

    fn set_str(&self, value: &str) -> Ev3Result<()> {
        Ok(())
    }

    pub fn get<T>(&self) -> Ev3Result<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Error,
    {
        let value = self.get_str()?;
        match value.parse::<T>() {
            Ok(value) => Ok(value),
            Err(e) => Err(Ev3Error::InternalError {
                msg: format!("{}", e),
            }),
        }
    }

    pub fn set<T>(&self, value: T) -> Ev3Result<()>
    where
        T: std::string::ToString,
    {
        Ok(())
    }

    #[inline]
    pub fn set_str_slice(&self, value: &str) -> Ev3Result<()> {
        Ok(())
    }

    pub fn get_vec(&self) -> Ev3Result<Vec<String>> {
        Ok(vec!("a".to_owned()))
    }

    pub fn get_raw_fd(&self) -> RawFd {
        self.file.borrow().as_raw_fd()
    }
}
