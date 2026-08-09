pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {

            // ❌ `self` is a `&` reference, so the data it refers to cannot be borrowed 
            // as mutable
            // self.sent_messages.push(String::from(message));

            // We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get 
            // a mutable reference to the value inside the RefCell<Vec<String>>, which is 
            // the vector. Then, we can call push on the mutable reference to the vector to 
            // keep track of the messages sent during the test

            // The borrow_mut returns the smart pointer type RefMut<T>

            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // To see how many items are in the inner vector, we call borrow on the RefCell<Vec<String>>
        // to get an immutable reference to the vector

        // The borrow method returns the smart pointer type Ref<T>

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
