use std::thread;

struct Rectangle {
    width: u32,
    height: u32,
}
    
impl Rectangle {
    fn area(&self)->u32{
        self.width * self.height
    }
}


fn main() {

    let rect1=Rectangle{width:30,height:50};
    println!("rect1 area is {}",rect1.area());

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}", y);

    let a = [1, 2, 3, 4, 5];
    let b = ["a", "b", "c"];
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    let d = [3; 5];
    let first = a[0];
    let second = a[1];

    let mut a = [1, 2, 3];
   // a[0] = 4;
   
   let m = add(2,3);
   println!("{}",m);

   let a = 3;
   let number = if a > 0 { 1 } else { -1 };
   println!("number 为 {}", number);

   for i in b.iter(){
    println!("{}",i);
   }

   let arr =[1,2,3,4,5];
   let mut iter=arr.into_iter().peekable();
   while  let Some(val) =iter.next() {
       if val % 2==0{
        continue;
       }
       println!("{}",val);
   }
 
 let nums =vec![1,2,3,4,5];
 let handles = nums.into_iter().map(|num|{
    thread::spawn(move|| {
        num * 2
    })
 }).collect::<Vec<_>>();
 
 for handle in handles {
    let result = handle.join().unwrap();
    println!("thread :{}",result);
 }

 let s =String::from("Broadcast");
 let part1= &s[0..5];
 let part2=&s[5..9];
 println!("{}={}+{}",s,part1,part2)
}



fn add(a: i32, b: i32) -> i32 {
    return a + b;
}


