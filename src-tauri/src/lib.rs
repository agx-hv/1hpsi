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
fn fetch_psi(date: Option<String>) -> Result<PsiResponse, String> {
    use network::client::HttpsClient;
    use network::response::extract_body;
    use psi::parser::parse_psi_response;

    // Create HTTPS client for api-open.data.gov.sg
    let client = HttpsClient::new("api-open.data.gov.sg", 443)
        .map_err(|e| e.to_string())?;

    // Construct API path with optional date query parameter
    let path = match date {
        Some(date_value) if !date_value.trim().is_empty() => {
            format!("/v2/real-time/api/psi?date={}", date_value)
        }
        _ => "/v2/real-time/api/psi".to_string(),
    };

    // Send GET request to API and handle response
    let raw = client
        .get(&path)
        .map_err(|e| e.to_string())?;

    let json = extract_body(&raw)
        .map_err(|e| e.to_string())?;

    let parsed = parse_psi_response(&json)
        .map_err(|e| e.to_string())?;

    println!("Parsed PSI Response: {:#?}", parsed);

    // Return the parsed PSI response
    Ok(parsed)
}