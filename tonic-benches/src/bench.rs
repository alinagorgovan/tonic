#[macro_use]
extern crate bencher;
use tonic;
use tokio::runtime::{Builder};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use bencher::Bencher;

use tokio::net::UnixStream;
use tonic::transport::{Uri};
use tower::service_fn;


pub mod echo {
    tonic::include_proto!("echo");
}
use echo::echo_client::EchoClient;
use echo::{EchoRequest};

fn random_string(n: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric)
                .take(n)
                .collect()
}

macro_rules! bench {
    ($name:ident, $message_size:expr, $count:expr, $sock_type:expr) => {
        fn $name(b: &mut Bencher) {
            let mut rt = Builder::new()
                .basic_scheduler()
                .enable_all()
                .build()
                .expect("runtime");
            
            let message = random_string($message_size);

            b.iter(|| {
                for _ in 0..$count {
                    let _result = rt.block_on( async {
                                    let mut client = if $sock_type == "tcp" {
                                        EchoClient::connect("http://[::1]:50051").await?
                                    } else {
                                        let channel = tonic::transport::Endpoint::from_static("lttp://[::]:50051")
                                                        .connect_with_connector(service_fn(|_: Uri| {
                                                            let path = "/tmp/tonic/echo_sock";    
                                                            UnixStream::connect(path)
                                                        }))
                                                        .await?;
                                        EchoClient::new(channel)
                                    };
                                    let request = tonic::Request::new(EchoRequest {
                                        message: message.clone(),
                                    });
                                    client.unary_echo(request).await?;
                                    Ok::<(), Box<dyn std::error::Error>>(())
                                });
                    }
                });
        }
    }
}

bench!(message_size_1kb_tcp, 1024, 1, "tcp");
bench!(message_size_100kb_tcp, 100 * 1024, 1, "tcp");
bench!(message_size_1mb_tcp, 1000 * 1024, 1, "tcp");

bench!(message_size_1kb_uds, 1024, 1, "uds");
bench!(message_size_100kb_uds, 100 * 1024, 1, "uds");
bench!(message_size_1mb_uds, 1000 * 1024, 1, "uds");

benchmark_group!(tcp, message_size_1kb_tcp, message_size_100kb_tcp, message_size_1mb_tcp);
benchmark_group!(uds, message_size_1kb_uds, message_size_100kb_uds, message_size_1mb_uds);

benchmark_main!(tcp, uds);