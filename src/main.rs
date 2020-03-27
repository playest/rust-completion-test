use ev3dev_lang_rust_derive::{Device};

#[derive(Clone)]
pub struct Driver { }

impl Driver {
    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        Attribute { }
    }
}
#[derive(Debug, Clone)]
pub struct Attribute { }

pub trait Device {
    fn get_attribute(&self, name: &str) -> Attribute;
}

#[derive(Clone, Device)]
struct Thingy {
    driver: Driver,
}

fn main() -> Result<(), ()> {
    let t = Thingy { driver: Driver{ } }; // will fail if executed because there is not driver named "a" but it doesn't matter, we are just testing completion here
    // t.get<ctrl + space> should at least list get_attribute (from trait Device), currently it doesn't
    // what's weird is that is lists .clone which is also obtained via #derive

    //t.get
    
    // the next line is commented (even if it works) because some IDE use existing symbols for their completion algorithm
    //t.get_attribute("a"); // what's weird is that it detects that get_attribute exists since this line is not an error
    //t.method_that_do_not_exists(); // and this gives an error

    Ok(())
}
