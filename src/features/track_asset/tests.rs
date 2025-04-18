use super::model::{GeofenceStatus, Location, Movement};
use super::logic::{check_geofence, compare_status};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_geofence_inside() {
        let loc = Location { lat: 41.0, lon: -73.0 };
        let status = check_geofence(&loc);
        assert_eq!(status, GeofenceStatus::Inside);
    }

    #[test]
    fn test_check_geofence_outside() {
        let loc = Location { lat: 39.5, lon: -75.0 };
        let status = check_geofence(&loc);
        assert_eq!(status, GeofenceStatus::Outside);
    }

    #[test]
    fn test_movement_entered() {
        let old = Some(GeofenceStatus::Outside);
        let new = GeofenceStatus::Inside;
        let movement = compare_status(old, new);
        assert_eq!(movement, Movement::Entered);
    }

    #[test]
    fn test_movement_exited() {
        let old = Some(GeofenceStatus::Inside);
        let new = GeofenceStatus::Outside;
        let movement = compare_status(old, new);
        assert_eq!(movement, Movement::Exited);
    }

    #[test]
    fn test_movement_stayed_inside() {
        let old = Some(GeofenceStatus::Inside);
        let new = GeofenceStatus::Inside;
        let movement = compare_status(old, new);
        assert_eq!(movement, Movement::StayedInside);
    }

    #[test]
    fn test_movement_unknown_to_inside() {
        let old = None;
        let new = GeofenceStatus::Inside;
        let movement = compare_status(old, new);
        assert_eq!(movement, Movement::Unknown);
    }
}