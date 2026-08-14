
// There is no way to define a struct that inherits the parent struct’s fields and 
// method implementations without using a macro. Instead, Rust achieves the goals 
// of inheritance (interface contracts, polymorphism, and code reuse) using Traits

pub trait Averageable {
    fn add(&mut self, value: i32);
    fn remove(&mut self) -> Option<i32>;
    fn average(&self) -> f64;

    // Default trait implementation (like a base class virtual method)
    fn is_empty(&self) -> bool {
        self.average() == 0.0
    }
}

// We can use the pub keyword to decide which modules, types, functions, and methods
// in our code should be public, and by default everything else is private

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            average: 0.0,
        }
    }

    fn update_average(&mut self) {
        if self.list.is_empty() {
            self.average = 0.0;
            return;
        }
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

impl Averageable for AveragedCollection {

    fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn average(&self) -> f64 {
        self.average
    }
}
