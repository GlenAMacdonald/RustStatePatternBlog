// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

use RustStatePatternBlog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate food, yay");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate food, yay", post.content());
}
