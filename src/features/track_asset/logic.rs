use super::model::{GeofenceStatus, Location, Movement};

pub fn check_geofence(loc: &Location) -> GeofenceStatus {
    if loc.lat >= 40.0 && loc.lat <= 42.0 &&
        loc.lon >= -74.0 && loc.lon <= -72.0 {
        GeofenceStatus::Inside
    } else {
        GeofenceStatus::Outside
    }
}

pub fn compare_status(old: Option<GeofenceStatus>, new: GeofenceStatus) -> Movement {
    match (old, new) {
        (Some(GeofenceStatus::Outside), GeofenceStatus::Inside) => Movement::Entered,
        (Some(GeofenceStatus::Inside), GeofenceStatus::Outside) => Movement::Exited,
        (Some(GeofenceStatus::Inside), GeofenceStatus::Inside) => Movement::StayedInside,
        (Some(GeofenceStatus::Outside), GeofenceStatus::Outside) => Movement::StayedOutside,
        _ => Movement::Unknown,
    }
}
