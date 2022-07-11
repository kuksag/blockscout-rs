use actix_web::Responder;
use paperclip::actix::{api_v2_operation, web::HttpResponse};

#[api_v2_operation(
    summary = "Health check",
    description = "Returns a 200 response if the server is healthy"
)]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().finish()
}
