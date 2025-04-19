use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use super::model::*;
use super::logic::*;
use crate::shared::error::AppError;

pub async fn track_endpoint(
    input: web::Json<TrackInput>,
    db: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let asset_id = &input.asset_id;
    let location = Location {
        lat: input.lat,
        lon: input.lon,
    };

    let new_status = check_geofence(&location);

    let old_status: Option<String> = sqlx::query_scalar(
        "SELECT last_status FROM asset_status WHERE asset_id = $1"
    )
        .bind(asset_id)
        .fetch_optional(db.get_ref())
        .await
        .map_err(AppError::from)?;

    let parsed_old = old_status.and_then(|s| match s.as_str() {
        "Inside" => Some(GeofenceStatus::Inside),
        "Outside" => Some(GeofenceStatus::Outside),
        _ => None,
    });

    let movement = compare_status(parsed_old, new_status.clone());

    sqlx::query(
        "INSERT INTO asset_status (asset_id, last_status, updated_at)
         VALUES ($1, $2, now())
         ON CONFLICT (asset_id) DO UPDATE SET last_status = $2, updated_at = now()"
    )
        .bind(asset_id)
        .bind(format!("{:?}", new_status))
        .execute(db.get_ref())
        .await
        .map_err(AppError::from)?;

    Ok(HttpResponse::Ok().json(TrackOutput {
        asset_id: asset_id.clone(),
        movement: format!("{:?}", movement),
    }))
}
