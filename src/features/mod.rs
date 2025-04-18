pub mod track_asset;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/track")
            .route("", web::post().to(track_asset::track_endpoint))
    );
}
