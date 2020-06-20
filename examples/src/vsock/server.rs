use futures::Stream;
use tokio_vsock::{VsockListener};
use tonic::{transport::Server, Request, Response, Status, Streaming};
use nix::sys::socket::{SockAddr, VsockAddr};
use libc;
use std::pin::Pin;
use futures::stream::TryStreamExt;


pub mod echo {
    tonic::include_proto!("grpc.examples.echo");
}

use echo::echo_server::{Echo, EchoServer};
use echo::{EchoRequest, EchoResponse};

type EchoResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send + Sync>>;

#[derive(Default)]
pub struct EchoService {}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn unary_echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = format!("{}", request.into_inner().message);

        Ok(Response::new(EchoResponse { message }))
    }
    type ServerStreamingEchoStream = ResponseStream;

    async fn server_streaming_echo(
        &self,
        _: Request<EchoRequest>,
    ) -> EchoResult<Self::ServerStreamingEchoStream> {
        Err(Status::unimplemented("not implemented"))
    }

    async fn client_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<EchoResponse> {
        Err(Status::unimplemented("not implemented"))
    }

    type BidirectionalStreamingEchoStream = ResponseStream;

    async fn bidirectional_streaming_echo(
        &self,
        _: Request<Streaming<EchoRequest>>,
    ) -> EchoResult<Self::BidirectionalStreamingEchoStream> {
        Err(Status::unimplemented("not implemented"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listen_port = 8000;
    let mut listener = VsockListener::bind(&SockAddr::Vsock(VsockAddr::new(
        libc::VMADDR_CID_ANY,
        listen_port,
    )))
    .expect("unable to bind virtio listener");

    let greeter = EchoService::default();
    
    Server::builder()
        .add_service(EchoServer::new(greeter))
        .serve_with_incoming(listener.incoming().map_ok(vsock::VsockStream))
        .await?;

    Ok(())
}

mod vsock {
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite};
    use tonic::transport::server::Connected;

    #[derive(Debug)]
    pub struct VsockStream(pub tokio_vsock::VsockStream);

    impl Connected for VsockStream {}

    impl AsyncRead for VsockStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for VsockStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}