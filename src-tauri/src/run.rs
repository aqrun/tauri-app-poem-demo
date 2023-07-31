use anyhow::Result;
use tauri::Manager;
use poem::{get, listener::TcpListener, EndpointExt, Route, Server};
use tokio::time::Duration;
use crate::models::State;
use crate::controllers::server_index;
use crate::services;

///
/// 主应用启动
/// 
pub async fn run() -> Result<(), std::io::Error> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            // http 全局状态
            let state = State {
                main_window,
            };

            // 新进程开启HTTP服务
            tokio::spawn(async move {
                run_server(state).await
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            services::greet_command,
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");

    Ok(())
}

///
/// 监听HTTP服务
/// 
pub async fn run_server(state: State) -> Result<(), std::io::Error> {
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
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(Duration::from_secs(5)),
        )
        .await
}
