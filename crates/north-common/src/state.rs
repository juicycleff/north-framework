use syn::parse::ParseStream;

pub trait NorthStateDataClone {
    fn clone_box(&self) -> Box<dyn NorthStateData>;
}

impl<T> NorthStateDataClone for T
where
    T: 'static + NorthStateData + Clone,
{
    fn clone_box(&self) -> Box<dyn NorthStateData> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn NorthStateData> {
    fn clone(&self) -> Box<dyn NorthStateData> {
        self.clone_box()
    }
}

pub trait NorthStateData: NorthStateDataClone + Send + Sync {}
