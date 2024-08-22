use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
// 假设你正在开发一个博客系统，其中每个用户可以查看不同的文章页面。
//页面的渲染是一个计算密集型的过程，可能涉及数据库查询、模板渲染等操作。
//因此，为了优化性能，你决定在服务器端实现一个缓存系统。
// 要求:
//实现PageCache结构体:
//该结构体应缓存根据用户ID和文章ID渲染的页面。
//你需要为该结构体实现一个get_page方法，该方法接受用户ID和文章ID，并返回渲染后的页面内容。
//如果相同的用户ID和文章ID已经渲染过，则get_page应直接返回缓存的页面，而不是重新染。
//通用性:
//oPageCache应支持任意类型的用户ID(例如，u32或String)和文章ID。
//缓存的内容应为渲染后的HTML页面(String类型)。
struct PageCache<UserId, ArticleId>
where
    UserId: Eq + Hash + Clone,
    ArticleId: Eq + Hash + Clone,
{
    cache: Arc<Mutex<HashMap<(UserId, ArticleId), String>>>,
    renderer: Box<dyn Fn(&UserId, ArticleId) -> String + Send + Sync>,
}

impl<UserId, ArticleId> PageCache<UserId, ArticleId>
where
    UserId: Eq + Hash + Clone,
    ArticleId: Eq + Hash + Clone,
{
    pub fn new<F>(renderer: F) -> Self
    where
        F: Fn(&UserId, ArticleId) -> String + Send + Sync + 'static,
    {
        PageCache {
            cache: Arc::new(Mutex::new(HashMap::new())),
            renderer: Box::new(renderer),
        }
    }

    pub fn get_page(&mut self, user_id: UserId, article_id: ArticleId) -> String {
        let mut cache = self.cache.lock().unwrap();
        if let Some(page) = cache.get(&(user_id.clone(), article_id.clone())) {
            return page.clone();
        }

        // 渲染页面并缓存结果
        let page = (self.renderer)(&user_id, article_id.clone());
        cache.insert((user_id, article_id), page.clone());
        page
    }
}

fn main() {
    let mut page_cache = PageCache::new(|user_id: &String, article_id: u32| -> String {
        println!("Rendering page for user {} and article {}", user_id, article_id);
        format!("Rendered HTML content for user {} and article {}", user_id, article_id)
    });

    // 第一次调用，会执行页面渲染
    println!("{}", page_cache.get_page("user1".to_string(), 42));//输出"Rendering page for user1 and article 42" 和 "Rendered HiML content for user user1 and article 42"
    // 第二次调用，直接返回缓存结果
    println!("{}", page_cache.get_page("user1".to_string(), 42));//输出 "Rendered HTL content for user user1 and article 42",不再渲染
    // 不同用户查看同一文章，会重新渲染
    println!("{}", page_cache.get_page("user2".to_string(), 42)); // 输出 "Rendering page for user2 and article 42" "endered Himl content for user user2 and article 42"
}
