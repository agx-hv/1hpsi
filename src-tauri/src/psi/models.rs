use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PsiResponse {
    pub code: i32, // HTTP status code
    pub data: Data,
    #[serde(rename = "errorMsg")] // rename to match JSON field
    pub error_msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    #[serde(rename = "regionMetadata")] // rename to match JSON field
    pub region_metadata: Vec<RegionMetadata>, 
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegionMetadata {
    pub name: String,
    #[serde(rename = "labelLocation")]
    pub label_location: LabelLocation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LabelLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub date: String,
    #[serde(rename = "updatedTimestamp")]
    pub updated_timestamp: String,
    pub timestamp: String,
    pub readings: Readings,
}

// Main PSI readings structure containing all pollutant indices for different regions
#[derive(Debug, Deserialize, Serialize)]
pub struct Readings {
    pub o3_sub_index: RegionReadings,
    pub no2_one_hour_max: RegionReadings,
    pub o3_eight_hour_max: RegionReadings,
    pub psi_twenty_four_hourly: RegionReadings,
    pub pm10_twenty_four_hourly: RegionReadings,
    pub pm10_sub_index: RegionReadings,
    pub pm25_twenty_four_hourly: RegionReadings,
    pub so2_sub_index: RegionReadings,
    pub pm25_sub_index: RegionReadings,
    pub so2_twenty_four_hourly: RegionReadings,
    pub co_eight_hour_max: RegionReadings,
    pub co_sub_index: RegionReadings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegionReadings {
    pub west: i32,
    pub east: i32,
    pub central: i32,
    pub south: i32,
    pub north: i32,
}