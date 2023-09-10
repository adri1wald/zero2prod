use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");
    let port = config.application_port;
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr)?;
    run(listener)?.await
}
