use crate::registry::service_instance::ServiceInstance;

pub trait Registration<T>: ServiceInstance {
    fn get_service(&self) -> T;
}
