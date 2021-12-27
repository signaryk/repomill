//use git2::Repository;
use hyper::{Body, Response, Request, StatusCode, Method};
use std::collections::HashMap;
use std::sync::Arc;

use crate::repoconf::RepoConf;

pub async fn process(req: Request<Body>, _rc: Arc<Vec<RepoConf>>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(Body::from("Hello world\n"))),
        (&Method::POST, "/deploy") => deploy(req).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found\n"))
                .unwrap())
        }
    }
}

async fn deploy(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let bytes_stream = hyper::body::to_bytes(req).await?;
    let _data: HashMap<String, String> = serde_json::from_slice(&bytes_stream).unwrap();

    let res = Response::builder().status(StatusCode::default()).body(Body::from("Done.\n")).unwrap();

    Ok(res)
}