#![allow(dead_code)]
use std::ops::Deref;
use std::sync::Arc;

use aragog::DatabaseAccess;

// #[derive(Clone)]
pub struct BoxedArangoConnection {
    pub connection: Box<dyn DatabaseAccess + Send + Sync + 'static>,
}

impl BoxedArangoConnection {
    pub fn connection(&self) -> &dyn DatabaseAccess {
        self.connection.deref()
    }
}

#[derive(Clone)]
pub struct ArcArangoConnection {
    pub connection: Arc<dyn DatabaseAccess + Send + Sync + 'static>,
}

impl ArcArangoConnection {
    pub fn connection(&self) -> &dyn DatabaseAccess {
        self.connection.deref()
    }
}
