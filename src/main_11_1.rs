fn main() {
    //练习1:使用 Vec 实现一个简单的栈实现一个简单的栈(后进先出，LIFO)数据结构，支持push、pop 和 peek 操作。
    let mut vector: Vec<u32> = Vec::new();
    vector.push(1);
    vector.push(2);
    vector.push(3);
    println!("{:?}", vector);
    vector.pop();
    println!("{:?}", vector);
    if let Some(first) = vector.get(0) {
        println!("The first element is: {}", first);
    } else {
        println!("The index is out of bounds.");
    }
}
