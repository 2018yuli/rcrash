
/*
socks5Proxy -l 127.0.0.1
./target/debug/rcrash -l 127.0.0.1:8080
curl --socks5-hostname 127.0.0.1:1080 baidu.com
*/

fn hand(src_stream: &std::net::TcpStream) -> Result<(), Box<(dyn std::error::Error)>> {
    println!("src {}", src_stream.peer_addr().unwrap());
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