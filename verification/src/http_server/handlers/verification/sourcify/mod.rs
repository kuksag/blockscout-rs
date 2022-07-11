use actix_web::error::Error;
use paperclip::actix::{
    api_v2_operation,
    web::{self, Json},
};

use super::VerificationResponse;

pub use self::api::SourcifyApiClient;
use self::types::ApiRequest;

mod api;
mod metadata;
mod types;

#[api_v2_operation]
pub async fn verify(
    sourcify_client: web::Data<SourcifyApiClient>,
    params: Json<ApiRequest>,
) -> Result<Json<VerificationResponse>, Error> {
    let response =
        api::verify_using_sourcify_client(sourcify_client.into_inner(), params.into_inner())
            .await?;
    Ok(Json(response))
}
