
fn main() {
    let myarray = [1,2,3,4,5];
    println!("myarray = {:?}", myarray);


    let my_u32_array : [u32; 5] = [1,2,3,4,5];
    println!("my_u32_array[1] = {}", my_u32_array[1]);

    // let index = "5".parse::<usize>().unwrap();
    // println!("my_u32_array[{}] = {}",index, my_u32_array[index]);

    // 数组的初始化
    let mybuffer :[u32; 32 * 1024] = [0; 32*1024];
    println!("mybuffer[1024] = {}", mybuffer[1024]);

    // 数组元素的修改
    let mut array:[i32; 5] = [0; 5];
    array[4] = 1;
    println!("array = {:?}", array);

}