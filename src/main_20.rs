
struct Fibonacci {
    //在这里定义所需字段
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        //初始化结构体
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
       //实现逻辑
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;

        Some(self.current)
    }
}

fn main() {
    let fib = Fibonacci::new();

    for num in fib.take(10) {
        println!("{}", num);
    }
}

