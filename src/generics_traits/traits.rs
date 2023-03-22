use std::fmt::{Debug, Display};
// Traits are similar to interfaces in other languages, but are also
// allowed to provide default implementations for some or all of the
// methods. Traits can also be implemented for any type, not just
// types defined in the current crate.
trait PrettyPrint {
    // This is the method signature with default implementation
    fn pretty_print(&self) -> String {
        String::from("Pretty Print Default Implementation")
    }
    // This is a method signature
    fn pretty_author(&self) -> String;
    fn pretty_format(&self) -> String {
        format!("Content {}", self.pretty_author())
    }
}
struct NewsArticle {
    title: String,
    content: String,
    author: String,
}
impl PrettyPrint for NewsArticle {
    fn pretty_print(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
    fn pretty_author(&self) -> String {
        format!("By {}", self.author)
    }
}
struct Tweet {
    username: String,
    content: String,
    retweet: bool,
}
impl PrettyPrint for Tweet {
    fn pretty_print(&self) -> String {
        format!("@{} - {}", self.username, self.content)
    }
    fn pretty_author(&self) -> String {
        format!("Of @{}", self.username)
    }
}

struct Post {
    author: String,
    content: String,
}
impl PrettyPrint for Post {
    fn pretty_author(&self) -> String {
        format!("Post By {}", self.author)
    }
}

// We can use tratis as function parameters to make our code more
// generic and reusable for different types of data.
// This function will accept any type that implements the PrettyPrint
// The equivalent signature of the function named as trait bound syntax is:
// fn notify<T: PrettyPrint>(item: &T) {}
fn notify(item: &impl PrettyPrint) {
    println!("Trait as parameter: {}", item.pretty_format());
}

trait OtherTrait{
    fn other_trait(&self) -> String {
        String::from("Other Trait Default Implementation")
    }
}
impl OtherTrait for NewsArticle {}
impl OtherTrait for Tweet {}
impl OtherTrait for Post {}
// We can use multiple traits as function parameters to make our code more
// generic and reusable for different types of data.
// Trait bound syntax looks like this:
// fn notify_multiple_traits<T: PrettyPrint + OtherTrait>(item: &T) {}
fn notify_multiple_traits(item: &(impl PrettyPrint + OtherTrait)) {
    println!("Multiple Traits as parameter: {} {}", item.other_trait(), item.pretty_format());
}

// Where syntax is used to specify multiple trait bounds with multiple
// types. This is equivalent to the following function signature:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// Using trait bounds to conditionally implement another trait
trait ParseString{
    fn to_string(&self) -> String;
}
// Only implement the trait ParseString for types T
// that implement the Display and the Clone traits
impl<T: Display + Clone> ParseString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

pub fn play_with(){
    println!("Play with Traits!");
    let news = NewsArticle {
        author: String::from("Dickens"),
        title: String::from("This is the new book"),
        content: String::from("Content of the article"),
    };
    let tw = Tweet {
      username: "rcsolis".to_string(),
      retweet:false,
      content: "I'm a rusteacean".to_string(),
    };
    let post = Post{
        author: "Ricardo".to_string(),
        content: "This is a post".to_string(),
    };
    println!("News: {}", news.pretty_print());
    println!("Tweet: {}", tw.pretty_print());
    println!("Post: {}", post.pretty_print());
    println!("Format News: {}", news.pretty_format());
    println!("Format Tweet: {}", tw.pretty_format());
    println!("Format Post: {}", post.pretty_format());
    notify(&news);
    notify(&tw);
    notify(&post);
    notify_multiple_traits(&news);
    notify_multiple_traits(&tw);
    notify_multiple_traits(&post);
}