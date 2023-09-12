#![feature(impl_trait_in_assoc_type)]
use anyhow::anyhow;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use volo_gen::mini::redis::*;
pub struct S {
    pub db: Mutex<HashMap<String, Bytes>>,
}
pub struct Channel {
    pub db: Mutex<HashMap<String, Bytes>>,
}

#[volo::async_trait]
impl volo_gen::mini::redis::ItemService for S {
    async fn redis_command(
        &self,
        req: Request,
    ) -> ::core::result::Result<Response, ::volo_thrift::AnyhowError> {
        match req.request_type {
            RequestType::Get => {
                let key = req.key.unwrap().into_string();
                let db = self.db.lock().unwrap();
                let value = db.get(&key).unwrap();
                Ok(Response {
                    message: Some("ok".into()),
                    response_type: ResponseType::Ok,
                })
            }
            RequestType::Set => {
                let key = req.key.unwrap().into_string();
                // let value = req.value;
                let value = req.value.unwrap().into_bytes();
                let mut db = self.db.lock().unwrap();
                db.insert(key, value);
                Ok(Response {
                    message: Some("ok".into()),
                    response_type: ResponseType::Ok,
                })
            }
            RequestType::Del => {
                match self
                    .db
                    .lock()
                    .unwrap()
                    .remove(&req.key.unwrap().into_string())
                {
                    Some(_) => Ok(Response {
                        message: Some("ok".into()),
                        response_type: ResponseType::Ok,
                    }),
                    None => Ok(Response {
                        message: Some("not found".into()),
                        response_type: ResponseType::Message,
                    }),
                }
            }
            RequestType::Ping => Ok(Response {
                message: Some("PONG".into()),
                response_type: ResponseType::Message,
            }),
            RequestType::Subscribe => {
                // let mut res = Response{
                // 	message:Some("ok".into()),
                // 	response_type:ResponseType::Ok,
                // };
                // let key = req.key.unwrap().into_string();
                // let (tx,rx) = tokio::sync::broadcast::channel(10);
                // match self.map.read().unwrap().borrow().channel
                Ok(Response {
                    message: Some("unimplemented".into()),
                    response_type: ResponseType::Ok,
                })
            }
            RequestType::Publish => Ok(Response {
                message: Some("unimplemented".into()),
                response_type: ResponseType::Ok,
            }),
            // _ => Err(anyhow!("not support")),
        }
    }
}
#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}
pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

#[derive(Clone)]
pub struct FilterService<S>(S);
#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let msg = format!("{:?}", &req);
        tracing::info!("Received request {:?}", &req);
        if msg.contains("haha") {
            Err(anyhow!("not support").into())
        } else {
            self.0.call(cx, req).await
        }
    }
}
pub struct FilterLayer;
impl<S> volo::Layer<S> for FilterLayer {
    type Service = FilterService<S>;
    fn layer(self, inner: S) -> Self::Service {
        FilterService(inner)
    }
}
