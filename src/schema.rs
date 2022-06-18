use chrono::NaiveDate;
use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BoardGames {
    #[serde(default)]
    pub name: String,
    pub entries: Vec<BoardGame>,
}

#[derive(Debug, Deserialize)]
pub struct BoardGame {
    pub name: String,
    #[serde(default)]
    pub bgg_id: usize,
    #[serde(default = "default_rating")]
    pub rating: u8,
    #[serde(default)]
    #[serde(alias = "unplayed")]
    #[serde(alias = "brand_new")]
    pub shelf_of_shame: bool,

    #[serde(default = "default_time")]
    pub date_acquired: NaiveDate,
    #[serde(default = "default_time")]
    pub last_played: NaiveDate,
}

/// this is just a marker, to avoid needing an `Option` over the dates
fn default_time() -> NaiveDate {
    chrono::naive::MIN_DATE
}

/// this is just a marker, to avoid needing an `Option` over rating
fn default_rating() -> u8 {
    255
}
