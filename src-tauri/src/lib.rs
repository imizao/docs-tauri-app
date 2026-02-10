// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use base64::Engine;
use std::fs;
use std::io::ErrorKind;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 将 base64 内容写入用户选择的路径（用于导出 Excel）
#[tauri::command]
fn save_file(path: String, base64_content: String) -> Result<(), String> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&base64_content)
        .map_err(|e| e.to_string())?;
    fs::write(&path, bytes).map_err(|e| {
        match e.kind() {
            ErrorKind::PermissionDenied => "没有写入权限".to_string(),
            ErrorKind::NotFound => "路径不存在".to_string(),
            _ => e.to_string(),
        }
    })?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![greet, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
