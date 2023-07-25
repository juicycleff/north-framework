use north_common::utils::logger_utils::print_format;
use yansi::Paint;
use crate::NorthServiceOptions;
use crate::error::Error;

#[allow(dead_code)]
pub(crate) async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

pub type NorthResult<T> = Result<T, Error>;


#[allow(dead_code)]
pub fn print_server_info(opts: &NorthServiceOptions) {
    println!(
        "{}",
        Paint::default("North Configuration")
            .bold()
            .underline()
            .blink()
    );
    print_format("name", opts.name.as_ref().unwrap().as_str());
    print_format("version", opts.version.as_ref().unwrap().as_str());
    print_format("address", opts.address.as_ref().unwrap().as_str());
    print_format("port", opts.port.as_ref().unwrap().to_string().as_str());
    print_format("keep alive", opts.keep_alive.to_string().as_str());
    print_format(
        "read timeout",
        format!("{}{}", opts.read_timeout.to_string().as_str(), "s").as_str(),
    );
    print_format(
        "write timeout",
        format!("{}{}", opts.write_timeout.to_string().as_str(), "s").as_str(),
    );
    print_format(
        "write timeout",
        format!("{}{}", opts.write_timeout.to_string().as_str(), "s").as_str(),
    );
    if opts.graceful_shutdown {
        print_format("graceful shutdown", "enabled");
    } else {
        print_format("graceful shutdown", "disabled");
    }
}
