fn fix_incorrect_order() {
    cook_order();

    // We can construct relative paths that begin in the parent module, rather 
    // than the current module or the crate root, by using super at the start 
    // of the path
    super::deliver_order();
}

fn cook_order() {}

pub struct Breakfast {

    // We can make each field public or not on a case-by-case basis
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {

    // Note that because `back_of_house::Breakfast` has a private field, the struct 
    // needs to provide a public associated function that constructs an instance of 
    // `Breakfast`. If `Breakfast` didn’t have such a function, we couldn’t create 
    // an instance of `Breakfast` in `eat_at_restaurant`, because we couldn’t set the 
    // value of the private `seasonal_fruit` field in `eat_at_restaurant`

    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

// In contrast, if we make an enum public, all of its variants are then public
pub enum Appetizer {
    Soup,
    Salad,
}
