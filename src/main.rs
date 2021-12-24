#[macro_use]
extern crate serde_derive;

use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};

pub mod repoconf;
use repoconf::RepoConf;

mod deploy;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>>{
    let rc = match RepoConf::new() {
        Ok(rc) => Arc::new(rc),
        Err(e) => panic!("Error: {}", e),
    };

    println!("Number of repos: {}", rc.len());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let service = make_service_fn(move |_| {
        let rc = Arc::clone(&rc);
        async {
            Ok::<_, Infallible>(service_fn(move |req| {
                deploy::process(req, rc.to_owned())
            }))
        }   
    });

    let server = Server::bind(&addr).serve(service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
