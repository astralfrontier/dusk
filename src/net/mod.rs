use log::info;
use std::net::TcpStream;

pub async fn tcp_socket_listener(socket: TcpStream) {
    info!("Accepted new connection from {:?}", &socket);
    // TODO: read data from socket
    let _ = socket.shutdown(std::net::Shutdown::Both);
    info!("Closed connection from {:?}", &socket);
}
