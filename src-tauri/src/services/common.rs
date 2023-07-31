use crate::prelude::*;

///
/// greet 事件
/// 
pub fn emit_greet_event(main_window: &Window, name: &str) -> Result<String> {
    let payload = format!("这是由 {} 使用后台服务发送的欢迎信息！", name);
    main_window.emit("greet9527", payload)?;

    Ok(String::from(""))
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet_command(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}