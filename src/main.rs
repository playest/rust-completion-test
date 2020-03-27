mod ev3;

use ev3dev_lang_rust_derive::{Device};
use ev3::{Attribute, Device, Driver};

#[derive(Clone, Device)]
struct Thingy {
    driver: Driver,
}

fn main() -> Result<(), ()> {
    let t = Thingy { driver: Driver::new() }; // will fail if executed because there is not driver named "a" but it doesn't matter, we are just testing completion here
    // t.<ctrl + space> should at least list get_attribute (from trait Device), currently it doesn't
    // what's weird is that is lists .clone which is also obtained via #derive

    //t.

    Ok(())
}
