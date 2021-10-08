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
            if let Some(true) = self.state.as_ref().map(|v| v.can_edit()) {
                self.content.push_str(to_add);
            }
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

        pub fn reject(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.reject())
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, post: &'a Post) -> Option<&'a str> {
            None
        }

        fn reject(self: Box<Self>) -> Box<dyn State>;

        fn can_edit(&self) -> bool {
            false
        }
    }

    struct Draft {}

    struct PendingReview {
        reviewed_count: i32,
    }

    struct Published {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview { reviewed_count: 0 })
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn can_edit(&self) -> bool {
            true
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        // we need 2 approvals to publish
        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            self.reviewed_count += 1;
            if self.reviewed_count > 1 {
                Box::new(Published {})
            } else {
                self
            }
        }

        fn reject(self: Box<Self>) -> Box<dyn State> {
            Box::new(Draft {})
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

        fn reject(self: Box<Self>) -> Box<dyn State> {
            self
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
        assert_eq!(None, post.content());
        post.approve();
        assert_eq!(Some("I ate some very state food this morning"), post.content());
    }

    #[test]
    fn test_reject() {
        let mut post = Post::new();
        post.add_text("We test das reject");
        assert_eq!(None, post.content());

        post.request_review();
        assert_eq!(None, post.content());

        post.reject();
        assert_eq!(None, post.content());
        post.approve();
        assert_eq!(None, post.content());

        post.request_review();
        assert_eq!(None, post.content());
        post.approve();
        post.approve();
        assert_eq!(Some("We test das reject"), post.content());
    }

    #[test]
    fn test_add_in_draft() {
        let mut post = Post::new();
        post.add_text("This is the very first sentence.");

        // set to review, add text to it and approve and check the text did not change
        post.request_review();
        post.add_text("And this won't be in it");
        post.approve();
        post.approve();
        assert_eq!(Some("This is the very first sentence."), post.content());
    }

    #[test]
    fn test_reject_then_add_as_draft() {
        let mut post = Post::new();
        post.add_text("This is the very first sentence.");

        post.request_review();
        post.add_text("And this won't be in it.");

        post.reject();
        post.add_text(" The editor is amazing.");
        post.request_review();
        post.approve();
        post.approve();

        assert_eq!(Some("This is the very first sentence. The editor is amazing."), post.content());
    }
}
