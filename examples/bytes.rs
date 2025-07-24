use anyhow::Ok;
use byteorder::{ByteOrder, LittleEndian};
use bytes::{BufMut, BytesMut};

fn main() -> anyhow::Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello world!");
    // b"goodbye" 字节字符串字面量
    // [..] 是 切片操作符，把 [u8; 7] 转为切片类型 &[u8]
    buf.put_slice(&b"goodbye"[..]);
    // b"hello world!goodbye"
    println!("{:?}", buf);

    // fn put_i64(&mut self, n: i64) 将一个 8 字节的 有符号整数（i64） 以 大端序（big endian 最高位在前） 写入缓冲区
    //   --  bytes 库默认所有 put_* 方法都采用 网络字节序（network byte order），而网络字节序就是 大端序（Big Endian）
    // 0xdeadbeef = 3735928559_u32
    // 它是个 32 位（4字节）整数，但 put_i64() 要求写入的是 64位（8字节），所以 Rust 会自动补齐高位
    // 等价于 buf.put_i64(0x00000000deadbeef_i64); // 00 00 00 00 DE AD BE EF
    buf.put_i64(0xdeadbeef);
    // b"hello world!goodbye\0\0\0\0\xde\xad\xbe\xef"
    println!("{:?}", buf);

    LittleEndian::write_i64(&mut buf, 0xdeadbeef);
    // b"\xef\xbe\xad\xde\0\0\0\0rld!goodbye\0\0\0\0\xde\xad\xbe\xef"
    println!("{:?}", buf);

    let a = buf.split();
    // b"\xef\xbe\xad\xde\0\0\0\0rld!goodbye\0\0\0\0\xde\xad\xbe\xef"
    println!("{:?}", a);
    // b""
    println!("{:?}", buf);

    // a 变成只读
    let mut b = a.freeze();
    println!("{:?}", b);

    // binary_search 只能用于已排序的字节流，但你操作的是未排序的原始数据
    // let pos = b.binary_search(&b'!').expect("not found");
    let pos = b.iter().position(|&x| x == b'!').expect("not found");
    let c = b.split_to(pos + 1);
    println!("{:?} -- {:?}", c, b);
    Ok(())
}
