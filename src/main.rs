use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    return run()?.await;
}
