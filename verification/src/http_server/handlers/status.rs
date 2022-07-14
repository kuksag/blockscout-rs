use actix_web::{HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy"),
    ),
)]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().finish()
}
