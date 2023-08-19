#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrS {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    V8 { region: String, address: String },
}

fn main() {
    let home = IpAddrS {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrS {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    use_ipaddrs("home", &home);
    use_ipaddrs("loopback", &loopback);

    let e_home = IpAddr::V4(127, 0, 0, 1);
    let e_loopback = IpAddr::V6(String::from("::1"));
    let e_country = IpAddr::V8 {
        region: String::from("lk"),
        address: String::from(":1:"),
    };

    use_ipaddre("home", &e_home);
    use_ipaddre("loopback", &e_loopback);
    use_ipaddre("country", &e_country);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn use_ipaddrs(name: &str, e: &IpAddrS) {
    let _kind = &e.kind;
    let _address = &e.address;
    println!("{name} {:?}", e);
}

fn use_ipaddre(name: &str, e: &IpAddr) {
    match e {
        IpAddr::V8 { region, address } => {
            let _r = region;
            let _a = address;
        }
        _ => {}
    };

    println!("{name} {:?}", e);
}
