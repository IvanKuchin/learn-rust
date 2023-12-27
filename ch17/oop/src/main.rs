use std::str::FromStr;

struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(DraftPost {})),
            content: String::new(),
        }
    }

    fn add_text(&mut self, text: &str) {
        self.content = String::from_str(text).unwrap()
    }

    fn content(&self) -> &str {
        &self
            .state
            .as_ref()
            .unwrap()
            .content(self)
    }

    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

struct DraftPost {}

impl State for DraftPost {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReviewPost {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReviewPost {}

impl State for PendingReviewPost {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
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

fn main() {
    let mut post = Post::new();

    post.add_text("New blog started in 2023");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("New blog started in 2023", post.content());
}
