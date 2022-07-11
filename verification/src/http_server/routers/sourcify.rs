use paperclip::actix::web::{self};

use crate::{
    config::SourcifyConfiguration,
    http_server::handlers::sourcify::{self, SourcifyApiClient},
};

use super::Router;

pub struct SourcifyRouter {
    api_client: web::Data<SourcifyApiClient>,
}

impl SourcifyRouter {
    pub fn new(config: SourcifyConfiguration) -> Self {
        let api_client = SourcifyApiClient::new(
            config.api_url,
            config.request_timeout,
            config.verification_attempts,
        );
        Self {
            api_client: web::Data::new(api_client),
        }
    }
}

impl Router for SourcifyRouter {
    fn register_routes(&self, service_config: &mut web::ServiceConfig) {
        service_config
            .app_data(self.api_client.clone())
            .route("/verify", web::post().to(sourcify::verify));
    }
}
