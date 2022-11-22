use anyhow::{anyhow, Result};
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

pub mod common;
pub mod tunnels;

static BASE_URL: &str = "https://api.cloudflare.com/client/v4";

#[derive(Debug, Clone)]
pub enum Auth {
    ApiKey { key: String, email: String },
    ApiToken(String),
}

#[derive(Debug, Clone)]
pub struct Client {
    auth: Auth,
    account_id: String,
    base_url: String,
}

impl Client {
    pub fn new(account_id: String, auth: Auth) -> Self {
        Self {
            account_id,
            auth,
            base_url: BASE_URL.to_string(),
        }
    }

    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    fn add_auth(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        match &self.auth {
            Auth::ApiKey { key, email } => request
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key),
            Auth::ApiToken(token) => request.header("Authorization", format!("Bearer {}", token)),
        }
    }

    async fn do_request<R: DeserializeOwned, T: Serialize + ?Sized>(
        &self,
        url: &str,
        method: Method,
        body: Option<&T>,
    ) -> Result<(R, StatusCode)> {
        let url = format!("{}{}", self.base_url, url);

        let client = reqwest::Client::new();

        let mut request = self.add_auth(client.request(method.clone(), &url));

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request
            .send()
            .await
            .map_err(|e| anyhow!("failed to send request {} {}: {:#}", method, url, e))?;

        let status = response.status();

        let data = response
            .text()
            .await
            .map_err(|e| anyhow!("failed to read body {:#}", e))
            .and_then(|response_text| {
                serde_json::from_str::<R>(&response_text)
                    .map_err(|e| anyhow!("failed to parse json {:#}", e))
            })
            .map_err(|e| {
                anyhow!(
                    "Failed to read response (status code {}) {} {}: {}",
                    status,
                    method,
                    url,
                    e
                )
            })?;

        Ok((data, status))
    }
}

macro_rules! handle_cloudflare_response {
    ($response:expr) => {
        let (data, status) = $response;
        if !data.success {
            return Err(anyhow!(
                "Cloudflare API returned an error ({}): {:#?} {:#?}",
                status,
                data.errors,
                data.messages
            ));
        }

        return Ok(data.result.unwrap_or(Default::default()));
    };
}

pub(self) use handle_cloudflare_response;

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::server::conn::AddrStream;
    use hyper::service::{make_service_fn, service_fn};
    use hyper::{Body, Request, Response, Server};
    use std::sync::Arc;
    use std::{convert::Infallible, net::SocketAddr};
    use tokio::select;
    use tokio::sync::mpsc::{Receiver, Sender};
    use tokio::sync::Mutex;
    use tokio::task::JoinHandle;

    pub fn setup_client(
        client: &mut Client,
        port: u16,
    ) -> (
        Sender<Response<Body>>,
        Receiver<Request<Body>>,
        JoinHandle<()>,
    ) {
        client.set_base_url(format!("http://localhost:{}", port).to_string());

        let (request_send, request_recv) = tokio::sync::mpsc::channel::<Request<Body>>(1);
        let (response_send, response_resv) = tokio::sync::mpsc::channel::<Response<Body>>(1);

        // make a mock test server to test the client
        let server = tokio::spawn(async move {
            let request_send = Arc::new(Mutex::new(request_send));
            let response_resv = Arc::new(Mutex::new(response_resv));
            let addr = SocketAddr::from(([127, 0, 0, 1], port));

            // A `MakeService` that produces a `Service` to handle each connection.
            let make_service = make_service_fn(move |_: &AddrStream| {
                let request_send = request_send.clone();
                let response_resv = response_resv.clone();

                // Create a `Service` for responding to the request.
                let service = service_fn(move |req: Request<Body>| {
                    let request_send = request_send.clone();
                    let response_resv = response_resv.clone();
                    async move {
                        request_send.lock().await.send(req).await.unwrap();

                        Ok::<Response<Body>, Infallible>(
                            response_resv.lock().await.recv().await.unwrap(),
                        )
                    }
                });

                // Return the service to hyper.
                async move { Ok::<_, Infallible>(service) }
            });

            let server = Server::bind(&addr).serve(make_service);

            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        });

        (response_send, request_recv, server)
    }

    #[tokio::test]
    async fn test_invalid_json() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 11000);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.do_request::<(), ()>("/", Method::GET, None).await
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::GET);
                assert_eq!(request.uri().path(), "/");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");


                response_send.send(Response::new(Body::from("invalid json"))).await.unwrap();

                assert_eq!(format!("{}", handle.await.unwrap().unwrap_err()), format!("Failed to read response (status code 200 OK) GET http://localhost:11000/: failed to parse json expected value at line 1 column 1"));
            } => assert!(true),
        }
    }
}
