use crate::server::error::NorthError;

#[allow(dead_code)]
pub(crate) async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub type NorthResult<T> = Result<T, NorthError>;
