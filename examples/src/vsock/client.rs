pub mod echo {
    tonic::include_proto!("grpc.examples.echo");
}
use echo::echo_client::EchoClient;
use echo::{EchoRequest};

use nix::sys::socket::{SockAddr, VsockAddr};

use std::convert::TryFrom;
use tokio_vsock::{VsockStream};
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    let listen_port = 8000;
    let cid = 3;
    
    let addr = SockAddr::Vsock(VsockAddr::new(
        cid,
        listen_port
    ));
       
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            // Connect to a vsock socket
            VsockStream::connect(&addr)

        }))
        .await?;

    let mut client = EchoClient::new(channel);

    let request = tonic::Request::new(EchoRequest {
        message: "Hello".to_string(),
    });

    let response = client.unary_echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
