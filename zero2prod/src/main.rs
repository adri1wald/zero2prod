use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".to_owned(), "info".to_owned(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration.");

    let port = config.application_port;
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(addr)?;

    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
