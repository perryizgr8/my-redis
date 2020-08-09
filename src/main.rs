use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, ip_port) = listener.accept().await.unwrap();
        println!("ip and port {:?}", ip_port);
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
        println!("spawned");
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("{:?}", cmd);
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("{:?}", cmd);
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimpl {:?}", cmd),
        };
        connection.write_frame(&response).await.unwrap();
    }
}
