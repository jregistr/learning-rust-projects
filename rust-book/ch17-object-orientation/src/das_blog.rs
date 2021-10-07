mod for_the_post {

    pub struct Post {
        content: String,
        state: Option<Box<dyn State>>,
    }

    impl Post {
        pub fn new() -> Post {
            Post { content: String::new(), state: Some(Box::new(Draft {})) }
        }
    }

    // methods
    impl Post {
        pub fn content(&self) -> Option<&str> {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn add_text(&mut self, to_add: &str) {
            self.content.push_str(to_add);
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }

        pub fn approve(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.approve())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, post: &'a Post) -> Option<&'a str> {
            None
        }
    }

    struct Draft {}

    struct PendingReview {}

    struct Published {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn content<'a>(&self, post: &'a Post) -> Option<&'a str> {
            let as_ref = post.content.as_str();
            Some(as_ref)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::for_the_post::*;

    #[test]
    fn test_content_only_if_published() {
        let mut post = Post::new();

        post.add_text("I ate some very state food this morning");
        assert_eq!(None, post.content());

        post.request_review();
        assert_eq!(None, post.content());

        post.approve();
        assert_eq!(Some("I ate some very state food this morning"), post.content());
    }
}
