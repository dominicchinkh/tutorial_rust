//! # Cargo
//!
//! `Cargo` is a collection of utilities to make performing certain
//! calculations more convenient.

// Running cargo doc --open will build the HTML for your current crate’s 
// documentation (as well as the documentation for all of your crate’s 
// dependencies) and open the result in a web browser

pub mod kinds;
pub mod utils;

// Add `pub use` statements to re-export the items at the top level
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
