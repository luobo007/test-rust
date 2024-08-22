use std::collections::HashMap;  
  
fn main() {  
    //练习2:使用 HashMap 实现一个字频统计器编写一个程序，统计一个字符串中每个单词出现的频率
    let str = "a b b c c c";  
    let word_counts = count_word(str);  
  
    for (word, count) in word_counts {  
        println!("{}: {}", word, count);  
    }  
}

fn count_word(str:&str) -> HashMap<&str,usize>{
    let mut word_counts=HashMap::new();
    for word in str.split_whitespace(){
        *word_counts.entry(word).or_insert(0)+=1;
    }
    word_counts
}
