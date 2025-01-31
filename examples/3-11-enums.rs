enum IpAddr {
    IPv4(u8,u8,u8,u8),
    IPv6(u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8,u8),
}

fn main() {
    let localhost = IpAddr::IPv4(127, 0, 0, 1);
    
    match localhost {
        IpAddr::IPv4(a, b, c, d) => {
            println!("{} {} {} {}", a, b, c, d);
        }
        _ => {}
    }
}