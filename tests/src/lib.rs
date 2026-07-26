
//-----------------------------
// Running Tests Consecutively

// cargo test -- --test-threads=1

//-------------------------
// Showing Function Output

// cargo test -- --show-output

//----------------------
// Running Single Tests

// cargo test <test name>

//---------------------------------
// Filtering to Run Multiple Tests

// We can specify part of a test name, and any test whose name matches that value will be run
// cargo test add

// Note that the module in which a test appears becomes part of the test’s name, so we can run
// all the tests in a module by filtering on the module’s name

// If we want to run only the ignored tests
// cargo test -- --ignored

// Run all tests whether they’re ignored or not
// cargo test -- --include-ignored

//-------------------
// Integration tests

// We can run a particular integration test function
// cargo test --test integration_test

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // The tests module is a regular module that follows the usual visibility rules. Because 
    // the tests module is an inner module, we need to bring the code under test in the outer 
    // module into the scope of the inner module
    
    // We use a glob here, so anything we define in the outer module is available to this 
    // tests module

    use super::*;

    #[test]
    // #[ignore] // Add the #[ignore] line to the test we want to exclude
    fn adder_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        // Adding custom failure message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]

    // To make `should_panic` tests more precise, we can add an optional expected parameter to
    // the `should_panic` attribute. The test harness will make sure that the failure message 
    // contains the provided text.

    // #[should_panic]    
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests

    #[test]
    fn adder_works_with_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
