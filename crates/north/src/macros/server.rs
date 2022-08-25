#[cfg(not(feature = "tls"))]
#[macro_export]
macro_rules! serve {
    ($north:expr, $addr:expr, async |$server_builder:ident, $protocol:ident| $continue:expr) => {
        use std::net::ToSocketAddrs;
        let sock_addr = $addr
            .to_socket_addrs()
            .expect("Invalid server address")
            .next()
            .unwrap();
        let ($protocol, $server_builder) = ("http://", hyper::Server::bind(&sock_addr));
        $continue
    };
}
