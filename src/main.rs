mod models;
mod test;
mod hasher;
mod node;
mod expiration;
mod test_main;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>>{
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let make_service = make_service_fn(|_conn| {
        async {
            Ok::<_, hyper::Error>(service_fn(handle_request))
        }
    });

    let server = Server::bind(&addr).serve(make_service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())

}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible>{
    Ok(Response::new(Body::from("Hello World")))
}
