use paperclip::actix::web::ServiceConfig;

use self::{solidity::SolidityRouter, sourcify::SourcifyRouter};
pub use self::app::AppRouter;

mod app;
mod solidity;
mod sourcify;

pub trait Router {
    fn register_routes(&self, service_config: &mut ServiceConfig);
}

impl<T: Router> Router for Option<T> {
    fn register_routes(&self, service_config: &mut ServiceConfig) {
        if let Some(router) = self {
            router.register_routes(service_config)
        }
    }
}

pub fn configure_router(router: &impl Router) -> impl FnOnce(&mut ServiceConfig) + '_ {
    |service_config| router.register_routes(service_config)
}
