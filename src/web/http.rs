use std::sync::{Arc, Mutex};
use hyper::{Body, Method, Request, Response, StatusCode};
use anyhow::{anyhow, Result};
use hyper::header::{CONTENT_TYPE, HeaderValue};
use serde_json::Value;
use once_cell::sync::Lazy;
use crate::config::from_config::Config;

static APPLICATION_JSON: Lazy<HeaderValue> = Lazy::new(|| HeaderValue::from_static("application/json"));


pub async fn handle_http(req: Request<Body>, config: Arc<Mutex<Config>>) -> Result<Response<Body>> {
    match *req.method() {
        Method::POST if req.uri().path() == "/update" => update_config(req, config).await,
        Method::GET if req.uri().path() == "/get" => responds_value(config).await,
        _ => {
            mkreq("tbc")
            // need to make some html
        }
    }
}

async fn update_config(p0: Request<Body>, p1: Arc<Mutex<Config>>) -> Result<Response<Body>>{
    let body = hyper::body::to_bytes(p0.into_body()).await?;
    match serde_json::from_slice::<Value>(body.as_ref()) {
        Ok(v) => {
            p1.lock().unwrap().merge(&v.to_string())?;
            let x = p1.lock().unwrap().compute();
            // Ok()
            mkreq(&x.to_string())
        }
        Err(_) => {
            mkreq("Unable to parse req")
        }
    }
}
async fn responds_value(config: Arc<Mutex<Config>>) -> Result<Response<Body>> {
    let x = config.lock().unwrap().compute().to_string();
    mkreq(&x)
}
fn mkreq(msg: &str) -> Result<Response<Body>> {
    let builder = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(msg.to_string()));
    match builder {
        Ok(x) => {
            Ok(x)
        }
        Err(err) => {
            Err(anyhow!("Unable to make response: {err}"))
        }
    }
}