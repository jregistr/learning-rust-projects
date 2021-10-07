pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }
}

impl Post {
    fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    fn add_text(&mut self, to_add: &str) {
        self.content.push_str(to_add);
    }

    fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

impl PendingReviewPost {
    fn approve(self) -> Post {
        Post { content: self.content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline() {
        let mut post = Post::new();
        post.add_text("Today I ate some tasty food in the morning");

        let post = post.request_review();
        let post = post.approve();

        assert_eq!("Today I ate some tasty food in the morning", post.content);
    }
}
