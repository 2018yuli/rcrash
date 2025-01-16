
fn main() {
    
    // 切片：数组的引用，0拷贝
    // 切片：一个（起始位置usize，长度usize）
    let mut arr : [i32; 5] = [1, 2, 3, 4, 5];
    // &mut arr[0..3] 是一个 可变借用 的切片. 
    // Rust 的所有权和借用规则确保了数据在被修改时不会被其他地方读取，从而防止数据竞争
    let slice = &mut arr[0..3];
    slice[0] = 12;
    println!("slice[0] = {}, len = {}", slice[0], slice.len());

    arr[0] = 1;
    println!("arr[0] = {}", arr[0]);

    let last_2_element_slice = &arr[arr.len()-2..];
    println!("last_2_element_slice = {:?}", last_2_element_slice);

    let modify_slice = &mut arr[..];
    modify_slice[3] = 0;
    println!("modify_slice = {:?}", modify_slice);
    println!("arr = {:?}", arr);

}