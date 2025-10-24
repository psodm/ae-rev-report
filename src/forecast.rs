use serde::{de, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastRow {
    pub soldToCompany: String,
    pub projectManager: String,
    pub projectName: String,
    pub projectId: String,
    pub class: String,
    pub startDate: String,
    pub finishDate: String,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub contractTotalValue: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub contractRemainingValue: f64,
    pub currency: String,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month1LaborRevenueCommit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month2LaborRevenueCommit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month3LaborRevenueCommit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month4LaborRevenueCommit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month5LaborRevenueCommit: f64,
    #[serde(deserialize_with = "deserialize_float_from_empty_string")]
    pub month6LaborRevenueCommit: f64,
}

fn deserialize_float_from_empty_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(0.0) // Or handle as an error, or return a default value
        } else {
            s.parse().map_err(de::Error::custom)
        }
    }