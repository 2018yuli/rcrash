use std::collections::HashMap;

fn main() {
    let mut transcript: HashMap<&str, u32> = HashMap::new();
    
    transcript.insert("alice", 95);
    transcript.insert("bob", 92);

    // 使用引用的原因是 HashMap 不需要移动键的所有权，它只是需要借用键来查找对应的值
    match transcript.get(&"bob") {
        Some(data) => println!("bob's score: {:?}", data),
        _ => {},
    }

    transcript.remove(&"bob");

    match transcript.get(&"bob") {
        Some(data) => println!("bob's score: {:?}", data),
        _ => println!("bob not found."),
    }

    // 迭代
    for (name, score) in transcript.iter() {
        println!("transcript[{:?}] = {:?}", *name, *score);
    }

    // 迭代2
    for (&name, &score) in transcript.iter() {
        println!("transcript[{:?}] = {:?}", name, score);
    }

    // hashMap 的迭代的无序性
    transcript.insert("jack", 97);
    for (&name, &score) in transcript.iter() {
        println!("transcript[{:?}] = {:?}", name, score);
    }
}