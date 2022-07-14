use super::{configure_router, Router, SolidityRouter, SourcifyRouter};
use crate::{config::Config, http_server::handlers::{status}};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::http_server::handlers::verification::{solidity::{self, types::{MultiPartFiles}}};
use crate::http_server::handlers::verification::{VerificationResponse, VerificationStatus, VerificationResult};

pub struct AppRouter {
    solidity: Option<SolidityRouter>,
    sourcify: Option<SourcifyRouter>,
}

impl AppRouter {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let solidity = match config.solidity.enabled {
            false => None,
            true => Some(SolidityRouter::new(config.solidity).await?),
        };
        let sourcify = config
            .sourcify
            .enabled
            .then(|| SourcifyRouter::new(config.sourcify));
        Ok(Self { solidity, sourcify })
    }
}

#[derive(OpenApi)]
#[openapi(
    handlers(
        status::status,
        solidity::multi_part::verify,
    ),
    components(
        VerificationResponse,
        MultiPartFiles,
        VerificationResult,
        VerificationStatus,
    ),
    tags(),
    modifiers()
)]
pub struct ApiDoc;

impl Router for AppRouter {
    fn register_routes(&self, service_config: &mut web::ServiceConfig) {
        service_config
            .route("/health", web::get().to(status::status))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/schema.json", ApiDoc::openapi()))
            .service(
                web::scope("/api/v1")
                    .service(web::scope("/solidity").configure(configure_router(&self.solidity)))
                    .service(web::scope("/sourcify").configure(configure_router(&self.sourcify))),
            );
    }
}
