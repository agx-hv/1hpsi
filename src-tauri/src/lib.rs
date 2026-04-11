// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod network;
mod psi;
use psi::models::PsiResponse;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Fetches PSI data from the API, optionally for a specific date
/// returns a structured PsiResponse 
#[tauri::command]
fn fetch_psi() -> Result<PsiResponse, String> {
    use network::client::get;
    use psi::parser::parse_psi_response;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Construct API path 
    let epoch_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let path = format!("https://www.haze.gov.sg/api/airquality/jsondata/{}", epoch_ns);

    // Send GET request and read raw response
    let body = get(&path).map_err(|e| e.to_string())?;

    let parsed = parse_psi_response(&body)
        .map_err(|e| e.to_string())?;

    // println!("Parsed PSI Response: {:#?}", parsed);

    // Return the parsed PSI response
    Ok(parsed)
}