pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
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
            self.state = Some(s.approve())
        }
    }

    pub fn redact(&mut self) {
        if let Some(s) = self.state.as_mut() {
            let new_content = s.redact(self.content.clone());
            self.content = new_content;
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        let _ = post;
        ""
    }
    fn redact(&mut self, input: String) -> String;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn redact(&mut self, input: String) -> String {
        input
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

    fn redact(&mut self, input: String) -> String {
        let mut new_content = String::new();
        create_redaction(input, &mut new_content);
        new_content
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

    fn redact(&mut self, input: String) -> String {
        let mut new_content = String::new();
        create_redaction(input, &mut new_content);
        new_content
    }
}

fn create_redaction(input: String, new_content: &mut String) {
    for word in input.split_whitespace() {
        if word.len() == 5 {
            new_content.push_str("===== ");
        } else {
            new_content.push_str(word);
        }
        new_content.push_str(" ");
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.redact();
    assert_eq!("I ate a =====  for =====  =====  ", post.content());

    let mut post_2 = Post::new();
    post_2.add_text("I am a big fan of the movie The Shining");

    post_2.redact();
    assert_eq!("", post_2.content());
}
