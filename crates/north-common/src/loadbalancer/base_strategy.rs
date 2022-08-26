/// #### BaseStrategy
/// Base trait for load balancers
pub trait BaseStrategy<T> {
    fn init(service_name: String, list: Vec<String>);

    /// Pick a service instance
    fn choose() -> T;

    fn get_instance<K>(name: String, instance_type: K) -> K;
}
