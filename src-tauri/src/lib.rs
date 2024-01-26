// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn exit() {
    std::process::exit(0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![exit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
