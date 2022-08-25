/// Reactive client trait
pub trait ReactiveClient<T> {
    /// Reactively connects a client to server
    fn connect(self);

    /// Access the current instance of a client
    fn client(self) -> T;

    /// Closes the client connection
    fn close(self);
}
