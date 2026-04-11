use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PsiResponse {
    #[serde(rename = "MachineID")]
    pub machine_id: String,

    #[serde(rename = "Categories")]
    pub categories: Vec<String>,

    #[serde(rename = "ChartPM25")]
    pub chart_pm25: Chart,

    #[serde(rename = "Chart1HRPM25")]
    pub chart_1hr_pm25: Chart,

    #[serde(rename = "ChartPM10")]
    pub chart_pm10: Chart,

    #[serde(rename = "ChartSO2")]
    pub chart_so2: Chart,

    #[serde(rename = "ChartO3")]
    pub chart_o3: Chart,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chart {
    #[serde(rename = "DivId")]
    pub div_id: String,

    #[serde(rename = "North")]
    pub north: RegionSeries,

    #[serde(rename = "South")]
    pub south: RegionSeries,

    #[serde(rename = "East")]
    pub east: RegionSeries,

    #[serde(rename = "West")]
    pub west: RegionSeries,

    #[serde(rename = "Central")]
    pub central: RegionSeries,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegionSeries {
    #[serde(rename = "Data")]
    pub data: Vec<DataPoint>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataPoint {
    pub value: f64,

    #[serde(rename = "valueColor")]
    pub value_color: String,

    pub band: String,

    #[serde(rename = "dateTime")]
    pub date_time: String,
}