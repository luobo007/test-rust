use std::fmt::Display;

// 不需要改Item的定义
trait Item<T = String> {
    type Output: Display;
    fn summarize(&self) -> Self::Output;
}

// 不需要改Apple结构的定义
struct Apple {
    name: String,
}

impl Item for Apple {
    type Output = String;

    fn summarize(&self) -> String {
        self.name.to_string()
    }
}

// 不需要改Weibo结构的定义
struct Weibo {
    author: String,
    content: String,
}

impl Item for Weibo {
    type Output = String;

    fn summarize(&self) -> String {
        format!("@{}:{}", self.author, self.content)
    }
}

pub struct Container {
    items: Vec<Box<dyn Item<Output = String>>>,
}

impl Container {
    pub fn iterator(&self) {
        for item in &self.items {
            println!("{}", item.summarize());
        }
    }
}

fn main() {
    let apple = Apple {
        name: "Apple".to_string(),
    };
    let weibo = Weibo {
        author: "weibo".to_string(),
        content: "hi".to_string(),
    };

    let container = Container {
        items: vec![Box::new(apple), Box::new(weibo)],
    };
    container.iterator();
}
