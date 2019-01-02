// crate
//  └── sound
//      └── instrument
//         └── woodwind
//      └── voice
mod sound {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                // each `super` steps out of a single module
                super::super::super::breathe_in();
                super::super::breate_out();
            }
        }
    }

    mod voice {}

    fn breate_out() {}
}

fn breathe_in() {}

mod plant {
    pub struct Vegetable {
        pub name: String,
        _id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                _id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salsa,
    }
}

// absolute
use crate::sound::instrument;
// relative
use self::sound::instrument::woodwind;
mod performance_group {
    pub use crate::sound::instrument::woodwind;
    pub fn clarinet_trio() {
        woodwind::clarinet();
        woodwind::clarinet();
        woodwind::clarinet();
    }
}

use std::collections::HashMap;

use std::fmt;
use std::io;

fn func1() -> fmt::Result {
    Ok(())
}

fn func2() -> io::Result<()> {
    Ok(())
}

use std::io::Result as IOResult;

fn func3() -> IOResult<()> {
    Ok(())
}

use std::{cmp::Ordering, collections};

use std::char::{self, from_digit};

use std::env::*;

fn main() {
    crate::sound::instrument::woodwind::clarinet();

    sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    let _order1 = menu::Appetizer::Soup;
    let _order2 = menu::Appetizer::Salsa;

    instrument::woodwind::clarinet();
    woodwind::clarinet();

    performance_group::clarinet_trio();

    let mut map = HashMap::new();
    map.insert(1, 2);

    let _f1 = func1();
    let _f2 = func2();
    let _f3 = func3();

    performance_group::woodwind::clarinet();
}
