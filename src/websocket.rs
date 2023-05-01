use std::collections::HashMap;
use std::net::SocketAddr;

use async_trait::async_trait;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use ezsockets::axum::Upgrade;
use ezsockets::{Error, SessionExt};

type SessionID = u16;
type Session = ezsockets::Session<SessionID, ()>;

struct EchoServer {
    sessions: HashMap<SessionID, Session>,
    handle: ezsockets::Server<Self>,
}

#[async_trait]
impl ezsockets::ServerExt for EchoServer {
    type Session = EchoSession;
    type Call = ();

    async fn on_connect(
        &mut self,
        socket: ezsockets::Socket,
        address: SocketAddr,
        args: <Self::Session as SessionExt>::Args,
    ) -> Result<Session, Error> {
        tracing::info!("Connected to {address}");
        let id = address.port();
        let session = Session::create(|handle| EchoSession { id, handle }, id, socket);
        Ok(session)
    }

    async fn on_disconnect(&mut self, id: <Self::Session as SessionExt>::ID) -> Result<(), Error> {
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        let () = call;
        Ok(())
    }
}

struct EchoSession {
    handle: Session,
    id: SessionID,
}

struct ServerState {
    wsServer: ezsockets::Server<EchoServer>,
}

#[async_trait]
impl SessionExt for EchoSession {
    type ID = SessionID;
    type Args = ();
    type Call = ();

    fn id(&self) -> &Self::ID {
        &self.id
    }
    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        tracing::info!("received: {text}");
        self.handle.text(text);
        Ok(())
    }

    async fn on_binary(&mut self, _bytes: Vec<u8>) -> Result<(), Error> {
        unimplemented!()
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), Error> {
        let () = call;
        Ok(())
    }
}

pub fn websocket_router<S>() -> Router<S> {
    let (websocket_server, _) = ezsockets::Server::create(|handle| EchoServer {
        sessions: HashMap::new(),
        handle,
    });
    let app = Router::new()
        .route("/websocket", get(websocket_handler))
        .with_state(websocket_server);
    app
}

async fn websocket_handler(
    State(ws_server): State<ezsockets::Server<EchoServer>>,
    ezsocket: Upgrade,
) -> impl IntoResponse {
    ezsocket.on_upgrade(ws_server, ())
}
