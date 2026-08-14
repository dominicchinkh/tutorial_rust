
pub struct RustPost {
    content: String,
}

pub struct DraftRustPost {
    content: String,
}

impl RustPost {
    pub fn new() -> DraftRustPost {
        DraftRustPost {
            content: String::new(),
        }
    }

    // The only way to get a published RustPost instance that does have a content method 
    // defined is to call the approve method on a PendingReviewRustPost, and the only way 
    // to get a PendingReviewRustPost is to call the request_review method on a DraftRustPost
    
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftRustPost {

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewRustPost {
        PendingReviewRustPost {
            content: self.content,
        }
    }

    // Note that DraftPost does not have a content method defined! So now the program 
    // ensures that all posts start as draft posts, and draft posts don’t have their 
    // content available for display. Any attempt to get around these constraints will 
    // result in a compiler error
}

pub struct PendingReviewRustPost {
    content: String,
}

impl PendingReviewRustPost {

    pub fn approve(self) -> RustPost {
        RustPost {
            content: self.content,
        }
    }
}
