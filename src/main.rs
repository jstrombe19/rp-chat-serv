use tokio::net::TcpListener;
use tokio::prelude::*;
use serde_derive::*;

mod j_read;
mod j_write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
  name: String,
  tx: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMessage {
  mess: Option<Message>,
  since: Option<usize>,
}

fn main() {
  let addr = "127.0.0.1:8088".parse().unwrap();
  let lis = TcpListener::bind(&addr).expect("could not bind address");

  let fut = lis.incoming().for_each(|sock| {
    let (sock_r, sock_w) = sock.split();
    let rd = j_read::JRead::new(sock_r).for_each(|s| {
      let v:ServerMessage = serde_json::from_str(&s)?;
      println!("received: {:?}", v);
      Ok(())
    }).map_err(|_| ());
    tokio::spawn(rd);
    Ok(())
  }).map_err(|e| println!("Listening Err :{:?}", e));

  tokio::run(fut);

}
