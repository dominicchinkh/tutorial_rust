use crate::gui::screen::interface::Draw;

pub mod component;
pub mod interface;

pub struct Screen {

    // This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in 
    // for any type inside a Box that implements the Draw trait

    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
