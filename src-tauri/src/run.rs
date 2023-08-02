use anyhow::Result;
use tauri::{Manager, Event, Window};
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};
use tokio::time::Duration;
use tokio_util::sync::CancellationToken;
use tokio::sync::{broadcast, mpsc};
use std::future::Future;
use std::sync::{Arc, Mutex};
use crate::models::State;
use crate::controllers::server_index;
use crate::services;
use once_cell::sync::Lazy;

pub static SHUTDOWN: Lazy<Arc<Mutex<(broadcast::Sender<()>, broadcast::Receiver<()>)>>> = Lazy::new(|| {
    Arc::new(Mutex::new(broadcast::channel(1)))
});

pub static SHUTDOWN1: Lazy<Arc<Mutex<(mpsc::Sender<()>, mpsc::Receiver<()>)>>> = Lazy::new(|| {
    Arc::new(Mutex::new(mpsc::channel(1)))
});

///
/// 主应用启动
/// 
pub async fn run() -> Result<(), std::io::Error> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
            
            app.listen_global("start-server", move |_e| {
                start_server_event(main_window.to_owned());
            });

            app.listen_global("stop-server", stop_server_event);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            services::greet_command,
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");

    Ok(())
}

fn start_server_event(main_window: Window) {
    let (poem_shutdown_tx, mut poem_shutdown_rx) = mpsc::channel(1);
    let (poem_shutdown_tx, mut poem_shutdown_rx) = broadcast::channel(1);
    // http 全局状态
    let state = State {
        main_window,
    };

    let shutdown = SHUTDOWN1.clone();
    let mut rx = shutdown.lock().unwrap();

    tokio::spawn(async move {
        start_server(state, &mut rx.1).await;
    });
}

fn stop_server_event(_e: Event) {
    let shutdown_tx = SHUTDOWN.clone().lock().unwrap().0;
    shutdown_tx.send(()).unwrap();
}

async fn start_server(state: State, shutdown: &mut mpsc::Receiver<()>) {
    let (poem_shutdown_tx, mut poem_shutdown_rx) = broadcast::channel(1);

    tokio::select! {
        _ = shutdown.recv() => {
            println!("cancelled");
        },
        _ = run_server(state, &mut poem_shutdown_rx) => {
            println!("server is running!!!");
        }
    };

    println!("poem shutdown---send--");
    poem_shutdown_tx.send(()).unwrap();
}

///
/// 监听HTTP服务
/// 
async fn run_server(state: State, shutdown: &mut broadcast::Receiver<()>) -> Result<(), std::io::Error> {
    let address = "0.0.0.0";
    let port = "9876";

    let app = Route::new()
        .at("/", get(server_index))
        .data(state);

    println!("Playground: https://{}:{}", address, port);

    let listener = TcpListener::bind(format!("{}:{}", address, port));
    Server::new(listener)
        .run_with_graceful_shutdown(
            app,
            async move {
                println!("poem shutdown---");
                let _ = shutdown.recv().await;
                // let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await
}
