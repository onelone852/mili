use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Error, tungstenite::Message};

type Socket =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub struct Connection {
    sender: Mutex<SplitSink<Socket, Message>>,
    recver: Mutex<SplitStream<Socket>>,
}

impl Connection {
    pub async fn new(socket_url: &str) -> Result<Self, Error> {
        let (socket, _) = connect_async(socket_url).await?;
        let (sender, recver) = socket.split();
        Ok(Self {
            sender: Mutex::new(sender),
            recver: Mutex::new(recver),
        })
    }

    pub async fn change_socket(&self, socket_url: &str) -> Result<(), Error> {
        let (mut ori_sender, mut ori_recver, res) = tokio::join!(
            self.sender.lock(),
            self.recver.lock(),
            connect_async(socket_url)
        );
        let (socket, _) = res?;
        (*ori_sender, *ori_recver) = socket.split();
        Ok(())
    }

    pub async fn send(&self, msg: String) -> Result<(), Error> {
        self.sender.lock().await.send(Message::Text(msg)).await
    }

    pub async fn recv(&self) -> Option<Result<String, Error>> {
        let msg = self.recver.lock().await.next().await;
        msg.map(|res| res.and_then(Message::into_text))
    }
}
