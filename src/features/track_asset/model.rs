use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct TrackInput {
    pub asset_id: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Serialize)]
pub struct TrackOutput {
    pub asset_id: String,
    pub movement: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeofenceStatus {
    Inside,
    Outside,
    #[allow(dead_code)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Movement {
    Entered,
    Exited,
    StayedInside,
    StayedOutside,
    Unknown,
}
