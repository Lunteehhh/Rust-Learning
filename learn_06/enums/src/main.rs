fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    impl IpAddr {
        fn show(&self) {
            println!("kind: {:?}\naddress: {}", self.kind, self.address);
        }
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    home.show();

    loopback.show();
}
