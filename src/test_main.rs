#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use super::*;
    use hyper::{Client, Server};
    use hyper::service::{make_service_fn, service_fn};
    use crate::handle_request;

    #[tokio::test]
    async fn test_hello_world() {
        let addr = SocketAddr::from(([127, 0, 0, 1], 4000));

        let make_service = make_service_fn(|_conn| {
            async {
                Ok::<_, hyper::Error>(service_fn(handle_request))
            }
        });

        let server = Server::bind(&addr).serve(make_service);

        let server_task = tokio::spawn(async {
            server.await.unwrap();
        });

        // Give the server a little time to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Send a GET request to the server
        let client = Client::new();
        let uri = "http://localhost:4000".parse().unwrap();
        let resp = client.get(uri).await.unwrap();

        // The response should have a status code of 200 OK
        assert_eq!(resp.status(), 200);

        // The response body should be "Hello, World!"
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();
        assert_eq!(body_str, "Hello World");

        // Cancel the server task to stop the server
        server_task.abort();
    }
}
