use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Pm25Readings {
    pub date: String,
    pub updatedTimestamp: String,
    pub timestamp: String,

    #[serde(skip)]
    pub hourly: Pm25Hourly, 
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Pm25Hourly {
    pub north: f32,
    pub south: f32,
    pub west: f32,
    pub east: f32,
    pub central: f32,
}

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

use chrono::NaiveDateTime;
impl RegionSeries {
    pub fn output(&self) -> Vec<(String, String)> {
        let mut result = vec![];
        for dp in &self.data {
            let dt =
                NaiveDateTime::parse_from_str(&dp.proper_date_time(), "%d %b %Y %I:%M%p").unwrap();

            let iso_date_time = dt.format("%Y-%m-%dT%H:%M:%S.%3f").to_string();

            result.push((iso_date_time, dp.rounded_pm25_string()));
        }
        result
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataPoint {
    pub value: f32,

    #[serde(rename = "valueColor")]
    pub value_color: String,

    pub band: String,

    #[serde(rename = "dateTime")]
    pub date_time: String,
}

impl DataPoint {
    pub fn proper_date_time(&self) -> String {
        self.date_time.replace("PM", ":00PM").replace("AM", ":00AM")
    }

    pub fn rounded_pm25_string(&self) -> String {
        match self.psi_value() {
            Some(v) => v.round().to_string(),
            None => "N/A".to_string(),
        }
    }

    fn psi_value(&self) -> Option<f32> {
        crate::psi::conversions::pm25_to_psi(self.value)
    }
}
