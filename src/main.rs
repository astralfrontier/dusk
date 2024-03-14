use log::error;
use std::net::TcpListener;
use std::time::SystemTime;

mod net;

// TODO: configure this, e.g. default logging level, output file, etc.
fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        //.chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() {
    match setup_logger() {
        Ok(_) => true,
        Err(e) => {
            error!("Unable to initialize logger: {:?}", e);
            std::process::exit(1);
        }
    };

    // Bind the listener to the address
    match TcpListener::bind("127.0.0.1:6379") {
        // TODO: support for graceful shutdown
        Ok(listener) => loop {
            if let Ok((socket, _)) = listener.accept() {
                tokio::spawn(async move {
                    net::tcp_socket_listener(socket).await;
                });
            }
        },
        Err(e) => {
            error!("Unable to bind to TCP socket: {:?}", &e);
            std::process::exit(2);
        }
    };
}
