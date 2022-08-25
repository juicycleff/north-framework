use async_trait::async_trait;

/// Contract to register and deregister instances with a Service Registry.
#[async_trait]
pub trait ServiceRegistry {
    /// Registers a registration. A registration contains information about an instance, such as host and port.
    /// @param registration registration metadata
    async fn register(&self);

    ///  Deregister a registration.
    ///  @param registration registration metadata
    async fn deregister(&self);

    /// Close the registration.
    fn close(&self);

    /// Sets the status of the registration. The status values are determined by the
    /// individual implementations.
    fn set_status(&self, status: String);

    //Gets the status of the registration. The status values are determined by the
    //individual implementations.
    // fn get_status<T: ServiceInstance> (&self) -> T;
}
