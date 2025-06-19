
/*
socks5Proxy -l 127.0.0.1
./target/debug/rcrash -l 127.0.0.1:1080
curl --socks5-hostname 127.0.0.1:1080 baidu.com
*/

use std::io::{Read, Write};

fn hand(src_stream: &std::net::TcpStream) -> Result<(), Box<(dyn std::error::Error)>> {
    println!("src {}", src_stream.peer_addr().unwrap());

    let mut src_reader = src_stream.try_clone()?;
    let mut src_writer = src_stream.try_clone()?;

    let mut buf:Vec<u8> = vec![0x00; 256];
    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x05 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    let nauth = buf[0] as usize;
    src_reader.read_exact(&mut buf[0..nauth])?;

    // buf[0..nauth] 必须包含 0x00

    src_writer.write(&[0x05])?;
    src_writer.write(&[0x00])?;

    println!("greeting");

    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x05 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x01 {
        panic!("unreachable");
    }
    src_reader.read_exact(&mut buf[0..1])?;
    if buf[0] != 0x00 {
        panic!("unreachable");
    }

    src_reader.read_exact(&mut buf[0..1])?;
    let host = match buf[0] {
        0x01 => {
            src_reader.read_exact(&mut buf[0..4])?;
            std::net::Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3]).to_string()
        },
        0x03 => {
            src_reader.read_exact(&mut buf[0..1])?;
            let len = buf[0] as usize;
            src_reader.read_exact(&mut buf[0..len])?;
            String::from_utf8_lossy(&buf[0..len]).to_string()
        },
        0x04 => {
            src_reader.read_exact(&mut buf[0..16])?;
            std::net::Ipv6Addr::new(
                ((buf[0x00] as u16) << 8) | (buf[0x01] as u16),
                ((buf[0x02] as u16) << 8) | (buf[0x03] as u16),
                ((buf[0x04] as u16) << 8) | (buf[0x05] as u16),
                ((buf[0x06] as u16) << 8) | (buf[0x07] as u16),
                ((buf[0x08] as u16) << 8) | (buf[0x09] as u16),
                ((buf[0x0a] as u16) << 8) | (buf[0x0b] as u16),
                ((buf[0x0c] as u16) << 8) | (buf[0x0d] as u16),
                ((buf[0x0e] as u16) << 8) | (buf[0x0f] as u16),
            ).to_string()
        },
        _ => panic!("unreachable")
    };
    src_reader.read_exact(&mut buf[0..2])?;
    let port = ((buf[0x00] as u16) << 8) | (buf[0x01] as u16);
    let dst = format!("{}:{}", host, port);

    println!("dst {}", dst);

    let dst_stream = std::net::TcpStream::connect(&dst)?;
    let mut dst_reader = dst_stream.try_clone()?;
    let mut dst_writer = dst_stream.try_clone()?;

    src_writer.write(&[0x05])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x01])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x00])?;
    src_writer.write(&[0x00])?;

    std::thread::spawn(move || {
        std::io::copy(&mut src_reader, &mut dst_writer).ok();
    });
    std::io::copy(&mut dst_reader, &mut src_writer).ok();

    Ok(())
}

fn main() {
    let mut c_listener = String::from("127.0.0.1:1080");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("Sockets5 Proxy");
        ap.refer(&mut c_listener).add_option(&["-l", "--listen"], argparse::Store, "listen address");
        ap.parse_args_or_exit();
    }

    println!("Lisen and server on {}", c_listener);

    let listener = std::net::TcpListener::bind(c_listener.as_str()).unwrap();

    // 接收链接
    for stream in listener.incoming() {
        match stream {
            Ok(data) => {
                // 启动一个线程来处理 TCP 的链接
                std::thread::spawn(move || {
                    if let Err(err) = hand(&data) {
                        println!("error: {:?}", err);
                    }
                });
            },
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }

}