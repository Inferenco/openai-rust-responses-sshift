use crate::error::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct RealtimeClient {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl RealtimeClient {
    pub async fn connect(api_key: &str, model: &str) -> Result<Self> {
        let url = format!("wss://api.openai.com/v1/realtime?model={}", model);
        let mut request = url
            .as_str()
            .into_client_request()
            .map_err(|e| crate::Error::Mcp(format!("Invalid request: {}", e)))?;

        request.headers_mut().insert(
            "Authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );
        request
            .headers_mut()
            .insert("OpenAI-Beta", "realtime=v1".parse().unwrap());

        let (socket, _) = connect_async(request)
            .await
            .map_err(|e| crate::Error::Mcp(format!("Failed to connect: {}", e)))?;

        Ok(Self { socket })
    }

    pub async fn send_event(&mut self, event: Value) -> Result<()> {
        let message = serde_json::to_string(&event).map_err(|e| crate::Error::Json(e))?;
        self.socket
            .send(tokio_tungstenite::tungstenite::Message::Text(message))
            .await
            .map_err(|e| crate::Error::Mcp(format!("Failed to send message: {}", e)))?;
        Ok(())
    }

    pub async fn receive_event(&mut self) -> Result<Option<Value>> {
        match self.socket.next().await {
            Some(Ok(msg)) => {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    let event: Value =
                        serde_json::from_str(&text).map_err(|e| crate::Error::Json(e))?;
                    Ok(Some(event))
                } else {
                    Ok(None)
                }
            }
            Some(Err(e)) => Err(crate::Error::Mcp(format!("WebSocket error: {}", e))),
            None => Ok(None),
        }
    }
}
