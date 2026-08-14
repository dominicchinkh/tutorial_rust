use crate::averaged_collection::Averageable;
use crate::averaged_collection::AveragedCollection;
use crate::blog::Post;
use crate::gui::screen::component::{ Button, SelectBox };
use crate::gui::screen::Screen;
use crate::rust_blog::RustPost;

mod averaged_collection;
mod blog;
mod gui;
mod rust_blog;

fn main() {

    //----------------------------------------------
    // Characteristics of Object-Oriented Languages
    //----------------------------------------------

    {
        let mut collection = AveragedCollection::new();
        collection.add(10);
        collection.add(20);
        render_stats(&collection);
    }

    //------------------------------------------------------
    // Using Trait Objects to Abstract over Shared Behavior
    //------------------------------------------------------

    {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox::new(
                    75,
                    10,
                    vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                )),
                Box::new(Button::new(
                    50,
                    10,
                    String::from("OK"),
                )),
            ],
        };

        screen.run();
    }

    //------------------------------------------------
    // Implementing an Object-Oriented Design Pattern
    //------------------------------------------------

    {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }

    {
        let mut post = RustPost::new();

        post.add_text("I ate a salad for lunch today");

        let post = post.request_review();

        let post = post.approve();

        assert_eq!("I ate a salad for lunch today", post.content());
    }
}

// Polymorphic function accepting ANY type that implements `Averageable`
fn render_stats(item: &dyn Averageable) {
    println!("Current Average: {}", item.average());
}
