use std::fmt::Display;
pub trait Summary {
    fn summarize(&self) -> String{
        format!("read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Report {
    pub head_line: String,
    pub author: String
}

impl Summary for Report {}


pub fn notify(item: &impl Summary) {
    println!("summary: {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("summary1: {}", item1.summarize());
    println!("summary2: {}", item2.summarize());
}
pub fn notify3<T: Summary + Display>(item1: &T, item2: &T) {
    println!("summary1: {}", item1.summarize());
    println!("summary2: {}", item2.summarize());
}

fn some_function<T, U>(t: T, u: U) 
    where T: Display + Clone,
          U: Clone + std::fmt::Debug
{}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_index = 0;

    for (index, item) in list.iter().enumerate() {
        if *item > list[largest_index] {
            largest_index = index;
        }
    }

    &list[largest_index]
}

pub fn trait_demo() {
    let news_article = NewsArticle { headline:String::from("Special counsel asks judge to limit Trump’s public statements in federal election subversion case") ,
     location:String::from("New York"), 
     author:String::from("Katelyn Polantz and Holmes Lybrand"), 
     content:String::from("
     Justice Department prosecutors want US District Judge Tanya Chutkan to reel in Donald Trump’s public statements in the federal 2020 election interference case against him, asking her to place a court order limiting what he can say, according to a newly released filing.
     The request for a limited gag order, filed in recent days, represents the most direct response from prosecutors with special counsel Jack Smith’s office to the former president’s public statements to date. The Justice Department said the order is needed to protect the integrity of his trial in March. Trump has already been ordered not to intimidate potential witnesses or talk to them about the facts of the case.  
     In making their argument, prosecutors pointed to false public statements Trump previously made around the 2020 vote “to erode public faith in the administration of the election and intimidate individuals who refuted his lies.”") };

     let tweet1 = Tweet { username:String::from("Jane and Nill"),
     content:String::from("A son of the notorious drug lord Joaquin `El Chapo` Guzman arrived in the United States on Friday following his extradition from Mexico to face drug trafficking charges, officials said."), 
     reply:false, 
     retweet:false };

     let tweet2 = Tweet {
        username: String::from("John Doe"),
        content: String::from("I just finished reading \"The Lord of the Rings\" trilogy, and it was amazing!"),
        reply: false,
        retweet: true,
    };

    let report = Report{head_line: String::from("hello world!"), 
    author:String::from("Kobe")};

    // println!("news_article summary: {}", news_article.summarize());
    // println!("tweet summary: {}", tweet.summarize());
    // println!("report summary: {}", report.summarize());

    notify(&tweet1);
    notify2(&tweet1, &tweet2);


    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_largest() {
        let largest_num:i32  = 10000;
        let test_vec = vec![1, 2, 5, -3, 9, -100, 0, 10000, -124];
        assert_eq!(&largest_num, largest(&test_vec));
    }
}
