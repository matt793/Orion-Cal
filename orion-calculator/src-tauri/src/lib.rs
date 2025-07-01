// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod engine;

#[tauri::command]
fn calculate(expression: &str) -> Result<f64, String> {
    engine::evaluate(expression)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
