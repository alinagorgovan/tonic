use tonic;

pub mod echo {
    tonic::include_proto!("echo");
}

use tonic::{transport::Server, Request, Response, Status};

use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};


#[derive(Default)]
pub struct EchoService {}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = format!("{}", request.into_inner().message);

        Ok(Response::new(EchoResponse { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = EchoService::default();

    println!("EchoServer listening on {}", addr);

    Server::builder()
        .add_service(EchoServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}