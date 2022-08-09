use tonic::{transport::Server, Request, Response, Status};

use server_description::{
    simple_service_server::{SimpleService, SimpleServiceServer},
    SimpleMessage,
};

pub mod server_description {
    tonic::include_proto!("server_description");
}

#[derive(Debug, Default)]
pub struct ImplSimpleService {}

#[tonic::async_trait]
impl SimpleService for ImplSimpleService {
    async fn health_check(&self, request: Request<()>) -> Result<Response<()>, Status> {
        println!("health_check: {:?}", request);
        Ok(Response::new(()))
    }

    async fn hello_world(
        &self,
        request: Request<SimpleMessage>,
    ) -> Result<Response<SimpleMessage>, Status> {
        println!("hello_world: {:?}", request);
        Ok(Response::new(SimpleMessage {
            message: format!("Hello World and {}", request.get_ref().message),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let simple_service = ImplSimpleService::default();

    Server::builder()
        .add_service(SimpleServiceServer::new(simple_service))
        .serve(addr)
        .await?;

    Ok(())
}
