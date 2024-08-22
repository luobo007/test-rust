fn main(){
    test_lifetime()
}

fn test_lifetime(){
    let large=longest("a","ab");
    println!("large one is {large}")
}

fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
         x
    }else{
        y
    }
}