//use git2::Repository;
use hyper::{Body, Response, Request};
use std::convert::Infallible;
use std::sync::Arc;

use crate::repoconf::RepoConf;

pub async fn process(_b: Request<Body>, _rc: Arc<Vec<RepoConf>>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!\n")))
}