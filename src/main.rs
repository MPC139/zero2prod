use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let listener = TcpListener::bind("127.0.0.1:0").expect("Fail to bind ip");
    let ip = listener.local_addr().unwrap().ip();
    let port = listener.local_addr().unwrap().port(); // for Future use

    println!("http://{ip}:{port}");

    run(listener)?.await
}
