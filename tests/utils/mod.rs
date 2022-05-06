use std::net::SocketAddr;

//noinspection HttpUrlsUsage
pub fn url_for(socket: SocketAddr, path: &str) -> String {
    format!("http://{}{}", socket, path)
}
