use std::collections::HashMap;
use serde_json::{json, Value};
use psi::models::Pm25Readings;

// For development use only: to simulate blocking network request
#[allow(dead_code)]
#[tauri::command]
fn sleep() {
    std::thread::sleep(std::time::Duration::from_secs(5));
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
fn check_latest_release_version() -> String {
    if let Ok(mut res) = ureq::get("https://api.github.com/repos/agx-hv/1hpsi/releases/latest").call() {
        let body = &res.body_mut().read_to_string().unwrap_or_default();
        let json_value: Value = serde_json::from_str(body).unwrap_or_default();
        let tag_name = json_value.get("tag_name").unwrap_or(&json!("")).to_string();
        return tag_name.replace("\"","").trim_start_matches("v").to_string();
    }
    "".to_string()
}

#[tauri::command]
fn update() -> HashMap<String,Vec<HashMap<String, String>>> {
    let mut output = HashMap::new();
    let mut psi_vec = vec![];
    let mut pm25_vec = vec![];

    if let Ok(parsed) = fetch_psi() {
        let pm25_data = parsed.chart_1hr_pm25;
        let (north, south, east, west, central) = (
            pm25_data.north.output_pm25(),
            pm25_data.south.output_pm25(),
            pm25_data.east.output_pm25(),
            pm25_data.west.output_pm25(),
            pm25_data.central.output_pm25(),
        );
        for i in 0..24 {
            let mut record = HashMap::new();
            let (n, s, e, w, c) = (
                north[i].1.clone(),
                south[i].1.clone(),
                east[i].1.clone(),
                west[i].1.clone(),
                central[i].1.clone(),
            );
            let mut mean = 0.0;
            for v in [&n, &s, &e, &w, &c] {
                mean += str::parse::<f32>(v).unwrap_or(0.0) / 5.0;
            }
            record.insert("timestamp".to_string(), north[i].0.clone());
            record.insert("Overall".to_string(), mean.round().to_string());
            record.insert("North".to_string(), n);
            record.insert("South".to_string(), s);
            record.insert("East".to_string(), e);
            record.insert("West".to_string(), w);
            record.insert("Central".to_string(), c);
            pm25_vec.push(record);
        }

        let (north, south, east, west, central) = (
            pm25_data.north.output_psi(),
            pm25_data.south.output_psi(),
            pm25_data.east.output_psi(),
            pm25_data.west.output_psi(),
            pm25_data.central.output_psi(),
        );
        for i in 0..24 {
            let mut record = HashMap::new();
            let (n, s, e, w, c) = (
                north[i].1.clone(),
                south[i].1.clone(),
                east[i].1.clone(),
                west[i].1.clone(),
                central[i].1.clone(),
            );
            let mut mean = 0.0;
            for v in [&n, &s, &e, &w, &c] {
                mean += str::parse::<f32>(v).unwrap_or(0.0) / 5.0;
            }
            record.insert("timestamp".to_string(), north[i].0.clone());
            record.insert("Overall".to_string(), mean.round().to_string());
            record.insert("North".to_string(), n);
            record.insert("South".to_string(), s);
            record.insert("East".to_string(), e);
            record.insert("West".to_string(), w);
            record.insert("Central".to_string(), c);
            psi_vec.push(record);
        }
    }

    if let Ok(parsed) = fetch_current_pm25() {
        let mut record = HashMap::new();
        let (n, s, e, w, c) = (
            parsed.hourly.north,
            parsed.hourly.south,
            parsed.hourly.east,
            parsed.hourly.west,
            parsed.hourly.central,
        );
        let mut mean = 0.0;
        for v in [&n, &s, &e, &w, &c] {
            mean += v / 5.0;
        }
        
        let mut ts = parsed.timestamp.clone();
        ts = ts.replace("+08:00", ".000");

        record.insert("timestamp".to_string(), ts.clone());
        record.insert("Overall".to_string(), mean.round().to_string());
        record.insert("North".to_string(), n.round().to_string());
        record.insert("South".to_string(), s.round().to_string());
        record.insert("East".to_string(), e.round().to_string());
        record.insert("West".to_string(), w.round().to_string());
        record.insert("Central".to_string(), c.round().to_string());

        if ts == *pm25_vec.last().unwrap_or(&HashMap::new()).get("timestamp").unwrap_or(&"".to_string()) {
            pm25_vec.pop();
        } else {
            pm25_vec.remove(0);
        }

        pm25_vec.push(record);

        let mut record = HashMap::new();
        let (n, s, e, w, c) = (
            psi::conversions::pm25_to_psi(parsed.hourly.north).unwrap_or(0.0),
            psi::conversions::pm25_to_psi(parsed.hourly.south).unwrap_or(0.0),
            psi::conversions::pm25_to_psi(parsed.hourly.east).unwrap_or(0.0),
            psi::conversions::pm25_to_psi(parsed.hourly.west).unwrap_or(0.0),
            psi::conversions::pm25_to_psi(parsed.hourly.central).unwrap_or(0.0),
        );
        let mut mean = 0.0;
        for v in [&n, &s, &e, &w, &c] {
            mean += v / 5.0;
        }
        
        let mut ts = parsed.timestamp;
        ts = ts.replace("+08:00", ".000");

        record.insert("timestamp".to_string(), ts.clone());
        record.insert("Overall".to_string(), mean.round().to_string());
        record.insert("North".to_string(), n.round().to_string());
        record.insert("South".to_string(), s.round().to_string());
        record.insert("East".to_string(), e.round().to_string());
        record.insert("West".to_string(), w.round().to_string());
        record.insert("Central".to_string(), c.round().to_string());

        if ts == *psi_vec.last().unwrap_or(&HashMap::new()).get("timestamp").unwrap_or(&"".to_string()) {
            psi_vec.pop();
        } else {
            psi_vec.remove(0);
        }

        psi_vec.push(record);
    }
    output.insert("pm25".to_string(),pm25_vec);
    output.insert("psi".to_string(),psi_vec);
    output
}

mod network;
mod psi;
use psi::models::PsiResponse;

use tauri::Manager;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update,
            get_app_version,
            check_latest_release_version,
            // Uncomment for development
            //sleep, 
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Fetches PSI data from the API, optionally for a specific date
/// returns a structured PsiResponse
fn fetch_psi() -> Result<PsiResponse, String> {
    use network::client::get;
    use psi::parser::parse_psi_response;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Construct API path
    let epoch_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();

    let path = format!(
        "https://www.haze.gov.sg/api/airquality/jsondata/{}",
        epoch_ns,
    );

    // Send GET request and read raw response
    let body = get(&path).map_err(|e| e.to_string())?;
    let parsed = parse_psi_response(&body).map_err(|e| e.to_string())?;

    // Return the parsed PSI response
    Ok(parsed)
}

/// Fetch current PM2.5 values using new more timely API
fn fetch_current_pm25() -> Result<Pm25Readings, String> {
    use network::client::get;
    use psi::models::{
        Pm25Hourly,
        Pm25Readings
    };
    let path =  "https://api-open.data.gov.sg/v2/real-time/api/pm25";

    // Send GET request and read raw response
    let body = get(&path).map_err(|e| e.to_string())?;
    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();

    let item = &v["data"]["items"][0];
    let hourly = &v["data"]["items"][0]["readings"]["pm25_one_hourly"];
    let mut pm25_readings: Pm25Readings = serde_json::from_str(&item.to_string()).unwrap_or_default();

    let pm25_hourly: Pm25Hourly = serde_json::from_str(&hourly.to_string()).unwrap_or_default();
    pm25_readings.hourly = pm25_hourly;
    Ok(pm25_readings)
}
