use std::sync::Arc;

use actix_web::{App, HttpServer};
use paperclip::actix::OpenApiExt;

use crate::config::Config;

pub use self::routers::{AppRouter, configure_router, Router};

pub mod handlers;
mod routers;

pub async fn run(config: Config) -> std::io::Result<()> {
    let socket_addr = config.server.addr;
    log::info!("Verification server is starting at {}", socket_addr);
    let app_router = Arc::new(
        AppRouter::new(config)
            .await
            .expect("couldn't initialize the app"),
    );
    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .with_json_spec_at("/api/docs.json")
            .with_json_spec_v3_at("/api/docs_v3.json")
            .with_swagger_ui_at("/api/docs")
            .configure(configure_router(&*app_router))
            .build()
    })
    .bind(socket_addr)?
    .run()
    .await
}
