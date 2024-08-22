fn main() {
    //第一题
    let s = "hello word";
    //第二题
    let s: Box<str> = "hello,world".into();
    greetings(&s);
    //第三题
    let mut s1 = String::new();
    s1.push_str("hello,world");
    s1.push('!');
    println!("3:{}", s1);
    //第四题
    let mut s2 = String::from("hello");
    s2.push(',');
    s2.push_str("world");
    s2 += "!";
    println!("{}", s2);
    //第五题
    let s3 = String::from("I like dogs");
    let s4 = s3.replace("dogs", "cats");
    println!("{}", s4);
    //第六题
    let s5 = String::from("hello,");
    let s6 = String::from("world!");
    let s7 = s5 + &s6;
    println!("{}", s7);
    //第七题
    let s8 = "hello,world".to_string();
    greetgings2(s8);
    //第八题
    let s9 = "hello,world".to_string();
    let s10: &str = &s9;
    //第九题
    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you dong \x3F (\\x3F means ?) {}", byte_escape);
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL P\"";
    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );
    let long_string = "String literals
                 can span multiple lines.
                 The linebreak and indentaction there \
                 can be escaped too!";
    println!("{}", long_string);
    //第十题
    let raw_str = "Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);
    let long_delimiter = r###"Hello, "##""###;
    println!("{}", long_delimiter);
    //第十一题
    let s11 = String::from("hi,中国");
    let h = &s11[0..1];
    println!("{}", h);
    let h1 = &s11[3..6];
    println!("{}", h1);
    //第十二题
    for c in "你好，世界".chars() {
        println!("{}", c);
    }
}

fn greetings(s: &str) {
    println!("{}", s);
}

fn greetgings2(s: String) {
    println!("{}", s);
}
