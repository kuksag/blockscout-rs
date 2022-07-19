use paperclip::actix::{api_v2_operation, NoContent};

#[api_v2_operation(
    summary = "Health check",
    description = "Returns a 2xx response if the server is healthy",
    tags("Health"),
)]
pub async fn status() -> NoContent {
    NoContent
}
