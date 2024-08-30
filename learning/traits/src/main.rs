// Traits allow us to define the set of methods that share among different types :-
// In traits agar default implementation nhi hai toh humei inherit krte waqt implement krna pdega
// pr agar default implementation hai toh implement krna optional hai... agar implement kra toh woh
// default implementation of trait ko override kr dega.

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    //fn summarize(&self) -> String {
    //    format!("{}, by {}", self.headline, self.author)
    //}
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

// This is just a syntactical sugar over something called as Trait Bounds Below this you will find
// actual code.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// THis is the actual Trait Bounds syntax here we have limited the generic that it must belong to
// summary interface.
//pub fn notify<T: Summary>(item: &T) {
//    println!("Breaking news! {}", item.summarize());
//}

fn main() {
    let tweet = Tweet {
        username: String::from("@nodix"),
        content: String::from("The sky is falling"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Abhishek"),
        headline: String::from("The Sky is falling..."),
        content: String::from("The sky is not actually falling u dumb ass.."),
    };
    println!("Tweet Summary : {}", tweet.summarize());
    println!("Article Summary : {}", article.summarize());
    notify(&article);
}
