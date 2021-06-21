use std::convert::Infallible;

use std::sync::{Arc, Mutex};


use hyper::{Body, Error, Request, Response, Server};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};

use tokio::sync::oneshot;

async fn callback_fn(
    context: Arc<Mutex<Option<oneshot::Sender<String>>>>,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    return match req.uri().path() {
        "/callback" => {
            let token = req.uri().query().unwrap_or_default().replace("code=", "");
            context.lock().unwrap().take().unwrap().send(token).unwrap();
            Ok(Response::new(Body::from(
                "Received the authorization code.",
            )))
        }
        _ => Ok(Response::new(Body::from("order geht nicht"))),
    };
}

pub async fn await_authorization_code_callback() -> Result<String, Error> {
    let (kill_tx, kill_rx) = tokio::sync::oneshot::channel::<()>();

    let (value_tx, value_rx) = oneshot::channel::<String>();

    let safe_value_tx = Arc::new(Mutex::new(Some(value_tx)));

    let make_service = make_service_fn(move |_conn: &AddrStream| {
        let context = safe_value_tx.clone();
        let service = service_fn(move |req| callback_fn(context.clone(), req));
        async move { Ok::<_, Infallible>(service) }
    });

    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    let server = Server::bind(&address).serve(make_service);

    //Spawn task for running the server
    tokio::spawn(async {
        server
            .with_graceful_shutdown(async {
                kill_rx.await.ok();
            })
            .await
    });
    //Wait for result. emit shutdown. return
    let authorization_code = value_rx.await.unwrap();
    kill_tx.send(()).unwrap();
    Ok(authorization_code)
}
