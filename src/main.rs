use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection = PgConnection::connect(&configuration.database.database_name)
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    //let ip = listener.local_addr().unwrap().ip();
    //let port = listener.local_addr().unwrap().port(); // for Future use

    //println!("http://{ip}:{port}");
    println!("{address}");

    run(listener, connection)?.await
}
