use lazy_static::lazy_static;
use std::net::SocketAddr;
use std::sync::Arc;
use volo::FastStr;
use volo_example::*;
use volo_gen::mini::redis::*;
lazy_static! {
    static ref CLIENT: volo_gen::mini::redis::ItemServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini::redis::ItemServiceClientBuilder::new("mini-redis")
            .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}
// async fn redis_request(req: Request) -> Result<Response, ResponseError<_>> {

// }

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();
    // let mut cmd = ;
    let res = match args[1].clone().to_uppercase().as_str() {
        "GET" => Request {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Get,
            ttl: None,
        },
        "SET" => Request {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: Some(FastStr::from(Arc::new(args.remove(3)))),
            request_type: RequestType::Set,
            ttl: None,
        },
        "PING" => Request {
            key: None,
            value: None,
            request_type: RequestType::Ping,
            ttl: None,
        },
        "DEL" => Request {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Del,
            ttl: None,
        },
        "SUBSCRIBE" => Request {
            key: Some(FastStr::from(Arc::new(args.remove(2)))),
            value: None,
            request_type: RequestType::Subscribe,
            ttl: None,
        },
        "PUBLISH" => Request {
            // key: Some(FastStr::from(args[2].as_str())),
            key: None,
            // value: Some(FastStr::from(args[3].as_str())),
            value: None,
            request_type: RequestType::Publish,
            ttl: None,
        },
        _ => {
            println!(
                "USAGE:
                      mini-redis-cli [OPTIONS] <SUBCOMMAND>

                  FLAGS:
                      -h, --help       Prints help information
                      -V, --version    Prints version information

                  OPTIONS:
                          --host <hostname>     [default: 127.0.0.1]
                          --port <port>         [default: 6379]

                  SUBCOMMANDS:
                      get     Get the value of key
                      help    Prints this message or the help of the given subcommand(s)
                      set     Set key to hold the string value"
            );
            return;
        }
    };
    // let res = redis_request(res).await;
    let res = CLIENT.redis_command(res).await;
    match res {
        Ok(res) => {
            println!("res:{:?}", res);
        }
        Err(e) => {
            println!("err:{:?}", e);
            // Err(e)
        }
    }
}
