#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct CustomResponse {
  message: String,
}

#[tauri::command]
async fn message_from_rust(window: tauri::Window) -> Result<CustomResponse, String> {
  println!("Called from {}", window.label());
  Ok(CustomResponse {
    message: "Hello from rust!".to_string()
  })
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![message_from_rust])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}