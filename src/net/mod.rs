use futures_lite::StreamExt;
use futures_util::sink::SinkExt;
use log::info;
use nectar::{event::TelnetEvent, TelnetCodec};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub async fn tcp_socket_listener(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let socketinfo = match stream.peer_addr() {
        Ok(addr) => format!("{:?}", addr),
        _ => "UNKNOWN".to_owned(),
    };

    info!("Accepted new connection from {:?}", &socketinfo);

    let mut frame = Framed::new(stream, TelnetCodec::new(1024));

    frame
        .send(TelnetEvent::Message("--- Connected to Dusk ---".to_owned()))
        .await?;

    // In a real application, you would want to handle Some(Err(_)) and None
    // variants, but for this example we'll be succinct for simplicities sake.
    while let Some(Ok(msg)) = frame.next().await {
        match msg {
            // We'll keep it simple and only match against the Message event.
            TelnetEvent::Message(string) => {
                // Let's echo back what we received.
                frame.send(TelnetEvent::Message(string)).await?;
            }
            _ => break,
        }
    }

    info!("Closed connection from {:?}", &socketinfo);
    Ok(())
}
