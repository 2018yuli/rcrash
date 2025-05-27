

fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in  1..11 {
        v.push(i);
    }
    println!("{:?}", v);

    // 使用 宏来创建 向量
    let mut v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", v1);
    println!("v1.length = {}, v1.capacity {}", v1.len(), v1.capacity());

    // pop 操作
    println!("{:?}", v1.pop());
    println!("after pop : v1.length = {}, v1.capacity {}", v1.len(), v1.capacity());

    // 自动扩容
    v1.push(10);
    v1.push(11);
    println!("after extend : v1.length = {}, v1.capacity {}", v1.len(), v1.capacity());

    // 迭代操作
    for i in 0..v1.len() {
        print!("v1[{:?}] = {:?} \t", i, v1[i]);
    }
    println!("");

    // 累乘2
    for e in v1.iter_mut() {
        *e *=  2;
    }

    // 迭代操作2
    let mut v1_idx = 0;
    for e in v1.iter() {
        print!("v1[{:?}] = {:?} \t", v1_idx, *e);
        v1_idx+=1;
    }
    println!("");

    
}