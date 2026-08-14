pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {

            // This ensures that whenever we create a new instance of Post, it will
            // start out as a draft. Because the state field of Post is private, there
            // is no way to create a Post in any other state

            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn request_review(&mut self) {

        // We call the take method to take the Some value out of the state field and 
        // leave a None in its place because Rust doesn’t let us have unpopulated 
        // fields in structs. This lets us move the state value out of Post rather 
        // than borrowing it. Then, we’ll set the post’s state value to the result of 
        // this operation

        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    // The add_text method behavior doesn’t depend on the state the post is in, so it’s 
    // not part of the state pattern

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // We call the as_ref method on the Option because we want a reference to the value 
    // inside the Option rather than ownership of the value. Because state is an 
    // Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned. 
    // If we didn’t call as_ref, we would get an error because we can’t move state out of 
    // the borrowed &self of the function parameter

    // When we call content on the &Box<dyn State>, deref coercion will take effect on the & 
    // and the Box so that the content method will ultimately be called on the type that 
    // implements the State trait

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

// The state objects are Draft, PendingReview, and Published, and they will all implement
// the State trait

trait State {

    // self: Box<Self>: This syntax means the method is only valid when called on a Box 
    // holding the type. This syntax takes ownership of Box<Self>, invalidating the old 
    // state so that the state value of the Post can transform into a new state
    
    // To eliminate some of the duplication, we might try to make default implementations 
    // for the request_review and approve methods on the State trait that return self. 
    // However, this wouldn’t work: When using State as a trait object, the trait doesn’t 
    // know what the concrete self will be exactly, so the return type isn’t known at 
    // compile time
    
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // We’re taking a reference to a post as an argument and returning a reference to part 
    // of that post, so the lifetime of the returned reference is related to the lifetime of 
    // the post argument

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {

        // It returns itself because when we request a review on a post already in the 
        // PendingReview state, it should stay in the PendingReview state

        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
