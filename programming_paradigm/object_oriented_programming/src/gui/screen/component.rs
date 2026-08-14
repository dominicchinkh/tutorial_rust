use crate::gui::screen::interface::Draw;

/*
 *  A generic type parameter can be substituted with only one concrete type at a time, 
 *  whereas trait objects allow for multiple concrete types to fill in for the trait 
 *  object at runtime
 *
 *    pub struct Screen<T: Draw> {
 *        pub components: Vec<T>,
 *    }
 *    
 *    impl<T> Screen<T>
 *    where
 *        T: Draw,
 *    {
 *        pub fn run(&self) {
 *            for component in self.components.iter() {
 *                component.draw();
 *            }
 *        }
 *    }
 */

pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button");
    }
}

impl Button {
    pub fn new(width: u32, height: u32, label: String) -> Self {
        Self { width, height, label }
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box");
    }
}

impl SelectBox {
    pub fn new(width: u32, height: u32, options: Vec<String>) -> Self {
        Self { width, height, options }
    }
}
