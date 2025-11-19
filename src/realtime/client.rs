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
    /// # Panics
    /// Panics if the Authorization or OpenAI-Beta header values cannot be parsed.
    ///
    /// # Errors
    /// Returns an error if the WebSocket connection fails or the request is invalid.
    pub async fn connect(api_key: &str, model: &str) -> Result<Self> {
        let url = format!("wss://api.openai.com/v1/realtime?model={model}");
        let mut request = url
            .as_str()
            .into_client_request()
            .map_err(|e| crate::Error::Mcp(format!("Invalid request: {e}")))?;

        request.headers_mut().insert(
            "Authorization",
            format!("Bearer {api_key}").parse().unwrap(),
        );
        request
            .headers_mut()
            .insert("OpenAI-Beta", "realtime=v1".parse().unwrap());

        let (socket, _) = connect_async(request)
            .await
            .map_err(|e| crate::Error::Mcp(format!("Failed to connect: {e}")))?;

        Ok(Self { socket })
    }

    /// # Errors
    /// Returns an error if the event cannot be serialized or the message cannot be sent.
    pub async fn send_event(&mut self, event: Value) -> Result<()> {
        let message = serde_json::to_string(&event).map_err(crate::Error::Json)?;
        self.socket
            .send(tokio_tungstenite::tungstenite::Message::Text(message))
            .await
            .map_err(|e| crate::Error::Mcp(format!("Failed to send message: {e}")))?;
        Ok(())
    }

    /// # Errors
    /// Returns an error if a WebSocket error occurs or the received message cannot be deserialized.
    pub async fn receive_event(&mut self) -> Result<Option<Value>> {
        match self.socket.next().await {
            Some(Ok(msg)) => {
                if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
                    let event: Value = serde_json::from_str(&text).map_err(crate::Error::Json)?;
                    Ok(Some(event))
                } else {
                    Ok(None)
                }
            }
            Some(Err(e)) => Err(crate::Error::Mcp(format!("WebSocket error: {e}"))),
            None => Ok(None),
        }
    }
}
