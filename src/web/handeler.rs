use std::sync::{Arc, Mutex};
use anyhow::anyhow;
use hyper::service::{make_service_fn, service_fn};
use crate::config::from_config::Config;
use crate::web::http::{handle_http};

pub async fn init(config: Arc<Mutex<Config>>) -> anyhow::Result<()>{
    let make_svc = make_service_fn(|_conn| {
        let config =Arc::clone(&config);
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                Box::pin(handle_http(req, config.clone()))
            }))
        }
    });
    let addr = ([127, 0, 0, 1], 8080).into();
    let builder = match hyper::Server::try_bind(&addr) {
        Ok(b) => Ok(b),
        Err(_) => {
            Err(anyhow!("Unable to bind builder"))
        }
    };
    let builder = builder?;
    match builder.serve(make_svc).await {
        Ok(_) => {
            Ok(())
        }
        Err(_) => {
            Err(anyhow!("Unable tos tart server"))
        }
    }
}