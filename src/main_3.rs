fn main() {
    //第一题
    let n = 5;
    if n < 0 {
        println!("{} is negateive", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
    //第二题
    let big_n = if n < 10 && n > -10 {
        println!("数字太小，先增加10倍在说");
        10 * n
    } else {
        println!("数据太大，我们得让它减半");

        n / 2
    };
    println!("{} -> {}", n, big_n);
    //第三题
    for n in 1..101 {
        if n == 100 {
            println!("Never LET THIS run");
        }
    }
    //第四题
    let names = [String::from("liming"), String::from("zhangsan")];
    for name in &names {
        println!("{}", name);
    }
    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in numbers {}
    println!("{:?}", numbers);

    //第五题
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v)
    }
    //第六题
    let mut n = 1;
    while n < 10 {
        if n % 15 == 0 {
            println!("aaaaa");
        } else if n % 3 == 0 {
            println!("%3");
        } else if n % 5 == 0 {
            println!("%5");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
    //第七题
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }
    println!("{}", n);
    //第八题
    let mut n1 = 0;
    for i in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }
        break;
    }
    println!("第八题：{}", 66);
    //第九题
    let mut count = 0u32;
    println!("let's count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK,that's enough");
            break;
        }
    }
    println!("{}", count);

    //第十题
    let mut counter = 0;
    let reuslt = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("第十题{}", counter);
    //第十一题
    let mut count = 0;
    'ounter: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }
        count += 5;

        'inner2: loop {
            if count >= 30 {
                break 'ounter;
            }
            continue 'ounter;
        }
    }
    assert!(count == 30);
}
