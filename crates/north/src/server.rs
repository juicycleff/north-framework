use crate::service::NorthServiceOptions;
use crate::Error;
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub struct NorthServer {}

pub async fn start_server(service_option: &NorthServiceOptions) -> Result<(), Error> {
    let full_addr = format!(
        "{}:{}",
        service_option.address.clone().unwrap(),
        service_option.port.unwrap().to_string(),
    );
    println!("{}", full_addr);
    let addr: std::net::SocketAddr = full_addr.parse()?;

    let service = hyper::service::make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(hyper::service::service_fn(
                move |_: Request<Body>| async move {
                    Ok::<_, Infallible>(Response::new(Body::from(format!(
                        "Hello, {}!",
                        remote_addr
                    ))))
                },
            ))
        }
    });

    let server = hyper::Server::bind(&addr).serve(service);

    if !service_option.graceful_shutdown {
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
        return Ok(());
    }

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let graceful = server.with_graceful_shutdown(async {
        rx.await.ok();
    });

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    let _ = tx.send(());
    Ok(())
}
