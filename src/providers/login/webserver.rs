use axum::{extract::{Query, State},response::Html,routing::get,Router};
use serde::Deserialize;
use std::{net::SocketAddr,sync::Arc,};
use tokio::sync::{oneshot, Mutex};

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

type SharedSender = Arc<Mutex<Option<oneshot::Sender<String>>>>;

pub(crate) async fn start() -> String {
    println!("Webserver Started");

    let (tx, rx) = oneshot::channel::<String>();
    let state: SharedSender = Arc::new(Mutex::new(Some(tx)));

    let app = Router::new()
        .route("/callback", get(callback))
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 49152));

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    tokio::spawn(async move {
        axum::serve(
            tokio::net::TcpListener::bind(addr).await.unwrap(),
            app,
        )
        .with_graceful_shutdown(async {
            let _ = shutdown_rx.await;
        })
        .await
        .unwrap();
    });

    let code = rx.await.unwrap();

    let _ = shutdown_tx.send(());
    println!("Webserver Shutdown");

    code
}

async fn callback(State(state): State<SharedSender>,Query(params): Query<CallbackQuery>) -> Html<&'static str> {
    let mut sender = state.lock().await;

    if let Some(tx) = sender.take() {
        let _ = tx.send(params.code);
        Html("Login complete. You can close this window.")
    } else {
        Html("Callback already handled.")
    }
}