use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, ip_port) = listener.accept().await.unwrap();
        println!("ip and port {:?}", ip_port);
        tokio::spawn(async move {
            process(socket).await;
        });
        println!("spawned");
    }
}

async fn process(socket: TcpStream) {
    // Connection defined by mini-redis lets us read/write redis frames instead of bytes.
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("got: {:?}", frame);

        //respond with error
        let response = Frame::Error("unimpl".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
