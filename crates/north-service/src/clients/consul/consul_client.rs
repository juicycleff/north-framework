use crate::clients::consul::prelude::*;

#[derive(Clone)]
pub struct ConsulReactiveClient {
    client: ConsulClient,
}

impl ConsulReactiveClient {
    /// Singleton instance of the client
    pub fn new(options: ConsulConfig) -> &'static Mutex<ConsulReactiveClient> {
        static mut CLIENT: MaybeUninit<Mutex<ConsulReactiveClient>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            CLIENT.as_mut_ptr().write(Mutex::new(ConsulReactiveClient {
                client: ConsulClient::new(options),
            }));
        });

        unsafe { &*CLIENT.as_ptr() }
    }

    /// Singleton instance of the client
    pub fn new_old(options: ConsulConfig) -> ConsulReactiveClient {
        ConsulReactiveClient {
            client: ConsulClient::new(options),
        }
    }
}

impl ReactiveClient<ConsulClient> for ConsulReactiveClient {
    fn connect(self) {
        // TODO: Rex handle creating connection
    }

    fn client(self) -> ConsulClient {
        self.client
    }

    fn close(self) {
        // TODO: Rex handle closing connection
    }
}
