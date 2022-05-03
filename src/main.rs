use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("localhost:8000").expect("Could not bind port 8000");
    let addr = listener.local_addr().unwrap();

    println!("\n--> Starting server on: \x1b[1;34m{}\x1b[1;m", addr);
    return run(listener)?.await;
}
