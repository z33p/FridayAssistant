use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    http,
    service::{make_service_fn, service_fn},
    Body, Server,
};
use hyper::{Method, Request, Response, StatusCode};
use serde_json::json;

#[tokio::main]
pub async fn server_run() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // And a MakeService to handle each connection...
    let make_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_post)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn handle_post(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response;

    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            let json_data = json!({"message": "Hello, World!"});

            response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json_data.to_string().into_bytes()))
                .unwrap();
        }
        _ => {
            response = not_found(req);
        }
    };

    Ok(response)
}

fn not_found(_req: Request<Body>) -> http::Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
