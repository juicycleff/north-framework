use crate::clients::mdns::prelude::*;

#[derive(Debug)]
pub struct MdnsReactiveClient {
    client: async_zeroconf::Service,
}

impl MdnsReactiveClient {
    /// Singleton instance of the client
    pub fn new(name: &str, port: u16) -> &'static Mutex<MdnsReactiveClient> {
        static mut CLIENT: MaybeUninit<Mutex<MdnsReactiveClient>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            CLIENT.as_mut_ptr().write(Mutex::new(MdnsReactiveClient {
                client: async_zeroconf::Service::new(name, "_http._tcp", port),
            }));
        });

        unsafe { &*CLIENT.as_ptr() }
    }

    /// Singleton instance of the client
    pub fn new_old(name: &str, port: u16) -> MdnsReactiveClient {
        MdnsReactiveClient {
            client: async_zeroconf::Service::new(name, "_http._tcp", port),
        }
    }
}

impl ReactiveClient<async_zeroconf::Service> for MdnsReactiveClient {
    fn connect(self) {
        // TODO: Rex handle creating connection
    }

    fn client(self) -> async_zeroconf::Service {
        self.client
    }

    fn close(self) {
        // TODO: Rex handle closing connection
    }
}
