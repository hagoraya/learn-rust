pub fn run() {
    //Traits are like interfaces -> given a box what are its inputs and/or outputs

    //Defining a trait
    trait Summary {
        fn summarize(&self) -> String;
        //You can also define a deault trait
        fn default_summary(&self) -> String {
            "Read more..".to_string()
        }
    }

    //Implement a trait
    struct Tweet {
        username: String,
        tweet: String,
        likes: u32,
    }

    struct Article {
        headline: String,
        content: String,
    }

    //Add implentation for trait
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {} [{}]", self.username, self.tweet, self.likes)
        }
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{}: {}", self.headline, self.content)
        }
    }

    //Using a trait
    let twe = Tweet {
        username: "Harry".to_string(),
        tweet: "Hello World".to_string(),
        likes: 23,
    };

    let news_article = Article {
        headline: "FIRE!".to_string(),
        content: "sikee".to_string(),
    };

    println!("Summary: {}", twe.summarize());
    println!("Default summary: {}", twe.default_summary());

    //* Traits with functions

    //Trait as parameters (3 ways)
    //Following function will take a T that implements summary
    fn notify(item: impl Summary) -> String {
        format!("{}", item.summarize())
    }

    fn notify2<T: Summary>(item: T) -> String {
        format!("{}", item.summarize())
    }

    fn notify3<T>(item: T) -> String
    where
        T: Summary,
    {
        format!("{}", item.summarize())
    }

    println!("{}", notify3(news_article));

    //* Returning traits
}
