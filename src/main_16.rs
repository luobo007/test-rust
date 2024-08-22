use std::rc::Rc;  
use std::rc::Weak;  
use std::cell::RefCell;  
use std::vec::Vec;  
  
// 用户结构体  
#[derive(Debug)]  
struct User {  
    name: String,  
    friends: Vec<Weak<RefCell<User>>>,  
}  
  
impl User {  
    fn new(name: String) -> Rc<RefCell<User>> {  
        Rc::new(RefCell::new(User {  
            name,  
            friends: Vec::new(),  
        }))  
    }  
  
    // 添加朋友  
    fn add_friend(&mut self, other: &Rc<RefCell<User>>) {  
        // 添加对方为朋友，使用 Weak 避免循环引用  
        self.friends.push(Rc::downgrade(other));  
 
    }  
  
    // 展示朋友列表  
    fn display_friends(&self) {  
        println!("Friends of {} :", self.name);  
        for friend in self.friends.iter() {  
            if let Some(friend_borrowed) = friend.upgrade() {  
                println!("{}", friend_borrowed.borrow().name);  
            }  
        }  
    }  
}  
  
fn main() {  
    // 创建用户  
    let alice = User::new("Alice".to_string());  
    let bob = User::new("Bob".to_string());  
  
    // 添加朋友关系  
    alice.borrow_mut().add_friend(&bob);  
  
    // 展示 Alice 的朋友  
    alice.borrow().display_friends();  
  

}