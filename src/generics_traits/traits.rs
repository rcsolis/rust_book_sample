// Traits are similar to interfaces in other languages, but are also
// allowed to provide default implementations for some or all of the
// methods. Traits can also be implemented for any type, not just
// types defined in the current crate.
trait PrettyPrint {
    fn pretty_print(&self) -> String;
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
    println!("News: {}", news.pretty_print());
    println!("Tweet: {}", tw.pretty_print());
}