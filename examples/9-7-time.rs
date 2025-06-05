use std::time::{Duration, SystemTime};
use std::thread::sleep;

fn main() {
    
    // SystemTime 是系统时间
    // 通过系统调用请求操作系统返回的系统时间
    let now = SystemTime::now();
    println!("now = {:?}", now);
    // println!("now.tv_sec = {}", now.tv_sec);

    // timestamp 是从 1970年1月1日到现在的秒数
    let timestamp = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    println!("timestamp = {:?}", timestamp);

    sleep(Duration::from_secs(4));

    // ela 航运术语：船，从一个港口出发，到另一个港口的时间
    println!("ela = {:?}", now.elapsed().unwrap());

    let future = now.checked_add(Duration::from_secs(60));
    println!("future = {:?}", future);
    println!("future = {:?}", future.unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap());

}