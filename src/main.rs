use std::net::SocketAddr;
use axum_server::Handle;

mod routes;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let app = routes::initialize().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let handle = Handle::new();
    let server = axum_server::bind(addr)
        .handle(handle.clone())
        .serve(app.into_make_service());

    tokio::spawn(async move {
        println!("Serving...");
        let server_end = server.await;
        println!("Stopped serving");
        server_end
    });

    let mut terminate =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
    let mut interrupt =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).unwrap();

    let _ = tokio::select! {
        _ = terminate.recv() => "SIGTERM",
        _ = interrupt.recv() => "SIGINT"
    };
    handle.graceful_shutdown(None);
    println!("Gracefully exiting");
}
