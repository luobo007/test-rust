use std::clone;
use std::collections::HashMap;  
use std::vec::Vec; 
 //使用Vec和HashMap 实现一个简单的书籍库存管理系统实现一个书籍库存管理系统，可以添加书籍、查询库存、更新库存以及删除书籍。
fn main() {
    //创建实例
    let mut bookManager=BookManager::new();
    //添加书籍
    bookManager.add_Book("A00001".to_string(), "Rust".to_string(), 10);
    bookManager.add_Book("A00002".to_string(), "Java".to_string(), 20);
    bookManager.add_Book("A00003".to_string(), "c++".to_string(), 30);
    //打印
    bookManager.print_books();
    //更新库存
    if bookManager.update_book_stock("A00001", 40){
        println!("Updated Rust book quantity is Success.");  
    }
     //打印
    let book= bookManager.get_book_stock("A00001");
    println!("{:?}",book);  
    //删除书籍
    bookManager.delete_book_stock("A00003");
     //打印
     bookManager.print_books();

}

//书籍实体
#[derive(Debug, Clone, PartialEq)]  
struct Book {
    isbn: String,  //书编号
    name: String,  //书名
    quantity: u32, //数量
}

//书籍管理实体
struct BookManager {
    book_isbn: HashMap<String, Book>, //根据快速查找书籍
    book_list: Vec<Book>,             //按顺序保持书籍
}

impl BookManager {
    //创建一个新的库存
    fn new() -> Self {
        BookManager {
            book_isbn: HashMap::new(),
            book_list: Vec::new(),
        }
    }

    //添加书籍
    fn add_Book(&mut self, isbn: String, name: String, quantity: u32) {
        let book = Book {
            isbn:isbn.clone(),
            name,
            quantity,
        };
        self.book_isbn.insert(isbn.clone(), book.clone());
        self.book_list.push(book);
    }
    //查询库存
    fn get_book_stock(&self,isbn:&str) ->Option<&Book>{
        self.book_isbn.get(isbn)
     
    }
    

    //更新库存
    fn update_book_stock(&mut self,isbn:&str,quantity: u32)->bool {

        if let Some(book)=self.book_isbn.get_mut(isbn){
            book.quantity=quantity;
            let book2 = Book {
                isbn:book.isbn.clone(),
                name:book.name.clone(),
                quantity:book.quantity,
            };
          
            self.delete_book_stock(isbn);
            self.book_list.push(book2.clone());
            self.book_isbn.insert(isbn.to_string(), book2.clone());
            return true;
        }else{
             

            return false;
        }

    }

    //删除书籍
    fn delete_book_stock(&mut self,isbn:&str)->bool {
        let index_to_remove=self.book_list.iter().position(|book|book.isbn==*isbn);

        if let Some(index) = index_to_remove{
            self.book_list.remove(index);
            self.book_isbn.remove(isbn);
            return true;
        }
        false
    }
    //打印书籍
    fn print_books(&self){
        for book in &self.book_list  {
            println!("{:?}",book);
            
        }
    }
}
