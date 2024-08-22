fn main() {
    //第一题
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3, "s=3");
    //第二题
    print();
    //第三题
    never_return();
    //第四题
    get_option(1);
    //第五题
    let b = false;
    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for 'false',but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn print() {
    println!("hello,world");
}

fn never_return() -> ! {
    panic!("error");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {}
        _ => {}
    };
    never_return();
}
