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
    let channel = Endpoint::try_from("lttp://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            // Connect to a vsock socket
            let listen_port = 9000;
            VsockStream::connect(&SockAddr::Vsock(VsockAddr::new(
                libc::VMADDR_CID_ANY,
                listen_port,
            )))
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
