use std::net::SocketAddr;

//noinspection HttpUrlsUsage
pub(crate) fn url_for(socket: SocketAddr, path: &str) -> String {
    format!("http://{}{}", socket, path)
}
