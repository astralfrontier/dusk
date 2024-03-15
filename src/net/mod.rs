use log::{error, info};
use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

pub async fn tcp_socket_listener(socket: TcpStream) {
    info!("Accepted new connection from {:?}", &socket);
    let socket2 = socket.try_clone().unwrap();

    let mut reader = BufReader::new(&socket);
    let mut writer = BufWriter::new(&socket2);

    let _ = writer.write("--- Connected to Dusk ---\n".as_bytes());
    let _ = writer.flush();

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {
                if line.ends_with("\n") {
                    line.pop();
                    if line.ends_with("\r") {
                        line.pop();
                    }
                }
                info!("Got line: {}", line);
            }
            Err(e) => {
                error!("Error reading line: {:?}", e);
                break;
            }
        }
    }
    let _ = socket.shutdown(std::net::Shutdown::Both);
    info!("Closed connection from {:?}", &socket);
}
