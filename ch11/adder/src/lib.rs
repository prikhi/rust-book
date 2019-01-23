#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    use super::rect::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    use super::*;
    // assert_eq/assert_ne
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // custom error strings
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // expected panics
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(-42);
    }

    // testing results
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // running subset of tests (try `cargo test one_hundred` or `cargo test add`)
    // modules are part of test names, so we could also do `cargo test tests`
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // ignore tests during normal runs
    // use `cargo test -- --ignored` to run it
    #[test]
    #[ignore]
    fn expensive_test() {
        // some code that takes an hour to run
    }

    // can test non-public functions as well
    #[test]
    fn internal() {
        assert_eq!(4, _internal_adder(2, 2));
    }
}

pub mod rect {
    #[derive(Debug)]
    pub struct Rectangle {
        pub length: u32,
        pub width: u32,
    }

    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            )
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            )
        }
        Guess { _value: value }
    }
}

fn _internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
