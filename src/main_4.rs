fn main(){

    fn take_ownership(s:&String)->&String{
        s
    }
    
    let s1=String::from("hello");
    let s2 =take_ownership(&s1);

    println!("{}",s1);
    println!("{}",s2);
}

