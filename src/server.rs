use std::convert::Infallible;
use std::ops::Deref;

use std::sync::Arc;

use futures::{sink::SinkExt};
use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper_tungstenite::{HyperWebsocket, tungstenite};
use hyper_tungstenite::tungstenite::error::Error;
use tokio::sync::watch;
use tungstenite::Message;

use crate::playback_mananger::Playing;

pub async fn upgrade_websocket(
    state: Arc<watch::Receiver<Option<Playing>>>,
    request: Request<Body>,
) -> Result<Response<Body>, Error> {
    let (response, websocket) =
        hyper_tungstenite::upgrade(request, None).map_err(Error::Protocol)?;
    tokio::spawn(async move {
        if let Err(e) = serve_websocket(state, websocket).await {
            eprintln!("Error in websocket connection: {}", e);
        }
    });
    Ok(response)
}

async fn serve_websocket(
    state: Arc<watch::Receiver<Option<Playing>>>,
    websocket: HyperWebsocket,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut websocket = websocket.await?;
    let mut state_clone = state.deref().clone();
    while state_clone.changed().await.is_ok(){
        let json = serde_json::to_string(&state_clone.borrow().clone()).unwrap();
        websocket.send(Message::Text(json)).await.unwrap();
    }
    Ok(())
}

pub async fn websocket(currently_playing_receiver: watch::Receiver<Option<Playing>>) {
    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let arc = Arc::new(currently_playing_receiver.clone());
        let service = service_fn(move |req| upgrade_websocket(arc.clone(), req));
        async move { Ok::<_, Infallible>(service) }
    });

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 8081));
    let server = Server::bind(&address).serve(make_service);

    //Spawn task for running the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
