pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approval_count: i8,
}

enum StateName {
    Draft,
    PendingReview,
    Published
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            approval_count: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            if matches!(s.get_state(),StateName::Draft) {
                self.content.push_str(text);
            }
            self.state = Some(s);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            if self.approval_count == 2 {
                self.state = Some(s.approve())
            } else {
                self.approval_count += 1;
            }
        }
    }

}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn get_state(&self) -> StateName;
    // fn reject(self: Box<Self>) -> Box<dyn State>;
}

trait Reject {
    fn reject(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_state(&self) -> StateName {
        StateName::Draft
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn get_state(&self) -> StateName {
        StateName::PendingReview
    }
}

impl Reject for PendingReview{
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_state(&self) -> StateName {
        StateName::Published
    }
}

