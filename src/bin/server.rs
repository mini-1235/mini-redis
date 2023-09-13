#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use volo_example::*;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::mini::redis::ItemServiceServer::new(S {
        db: Mutex::new(std::collections::HashMap::new()),
    })
    .layer_front(LogLayer)
    .layer_front(FilterLayer)
    .run(addr)
    .await
    .unwrap();
}
