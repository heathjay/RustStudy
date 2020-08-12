use chapter22Pattern::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("i ate a salad");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("I ate a sala", post.content());
}
