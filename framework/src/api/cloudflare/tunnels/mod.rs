use super::{common::CloudflareResponse, handle_cloudflare_response, Client};
use anyhow::{anyhow, Result};
use reqwest::Method;

mod types;

pub use types::*;

impl Client {
    pub async fn list_tunnels(&self) -> Result<Vec<Tunnel>> {
        let url = format!("/accounts/{}/cfd_tunnel", self.account_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Vec<Tunnel>>, ()>(&url, Method::GET, None)
                .await?
        );
    }

    pub async fn create_tunnel(&self, req: TunnelRequest) -> Result<Tunnel> {
        let url = format!("/accounts/{}/cfd_tunnel", self.account_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, TunnelRequest>(
                &url,
                Method::POST,
                Some(&req)
            )
            .await?
        );
    }

    pub async fn update_tunnel(&self, tunnel_id: &str, req: TunnelRequest) -> Result<Tunnel> {
        let url = format!("/accounts/{}/cfd_tunnel/{}", self.account_id, tunnel_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, TunnelRequest>(
                &url,
                Method::PATCH,
                Some(&req)
            )
            .await?
        );
    }

    pub async fn get_tunnel(&self, tunnel_id: &str) -> Result<Tunnel> {
        let url = format!("/accounts/{}/cfd_tunnel/{}", self.account_id, tunnel_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, ()>(&url, Method::GET, None)
                .await?
        );
    }

    pub async fn delete_tunnel(&self, tunnel_id: &str) -> Result<Tunnel> {
        let url = format!("/accounts/{}/cfd_tunnel/{}", self.account_id, tunnel_id);

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Tunnel>, ()>(&url, Method::DELETE, None)
                .await?
        );
    }

    pub async fn list_tunnel_connections(&self, tunnel_id: &str) -> Result<Vec<TunnelConnection>> {
        let url = format!(
            "/accounts/{}/cfd_tunnel/{}/connections",
            self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<Vec<TunnelConnection>>, ()>(
                &url,
                Method::GET,
                None
            )
            .await?
        );
    }

    pub async fn clean_up_tunnel_connections(&self, tunnel_id: &str) -> Result<()> {
        let url = format!(
            "/accounts/{}/cfd_tunnel/{}/connections",
            self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<()>, ()>(&url, Method::DELETE, None)
                .await?
        );
    }

    pub async fn get_tunnel_token(&self, tunnel_id: &str) -> Result<String> {
        let url = format!(
            "/accounts/{}/cfd_tunnel/{}/token",
            self.account_id, tunnel_id
        );

        handle_cloudflare_response!(
            self.do_request::<CloudflareResponse<String>, ()>(&url, Method::GET, None)
                .await?
        );
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::{CloudflareError, CloudflareResponse};
    use super::super::{tests::setup_client, Auth, Client};
    use super::*;
    use hyper::{Body, Response, StatusCode};
    use tokio::select;

    #[tokio::test]
    async fn test_list_tunnels() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10000);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                {
                    let handle = {
                        let client = client.clone();
                        tokio::spawn(async move {
                            client.list_tunnels().await.unwrap()
                        })
                    };

                    let request = request_recv.recv().await.unwrap();

                    assert_eq!(request.method(), Method::GET);
                    assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel");
                    assert_eq!(request.uri().query(), None);
                    assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");

                    let mut resp = CloudflareResponse::<Vec<types::Tunnel>>::default();

                    let result = vec![types::Tunnel::default()];
                    resp.result = Some(result.clone());
                    resp.success = true;

                    response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                    assert_eq!(handle.await.unwrap(), result);
                }

                {
                    let handle = {
                        let client = client.clone();
                        tokio::spawn(async move {
                            client.list_tunnels().await
                        })
                    };

                    let request = request_recv.recv().await.unwrap();

                    assert_eq!(request.method(), Method::GET);
                    assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel");
                    assert_eq!(request.uri().query(), None);
                    assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");

                    let mut resp = CloudflareResponse::<Vec<types::Tunnel>>::default();

                    let result = vec![CloudflareError{
                        code: 1000,
                        message: "test".to_string(),
                    }];
                    resp.errors = result.clone();

                    let mut resp = Response::new(Body::from(serde_json::to_string(&resp).unwrap()));

                    *resp.status_mut() = StatusCode::BAD_REQUEST;

                    response_send.send(resp).await.unwrap();

                    assert_eq!(format!("{}", handle.await.unwrap().unwrap_err()), format!("Cloudflare API returned an error ({}): {:#?} {:#?}", StatusCode::BAD_REQUEST, result, Vec::<String>::new()));
                }
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_create_tunnel() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10001);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.create_tunnel(types::TunnelRequest{
                        name: "test".to_string(),
                        tunnel_secret: "secret".to_string(),
                    }).await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::POST);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");
                assert_eq!(request.headers().get("Content-Type").unwrap(), "application/json");

                let full_body = hyper::body::to_bytes(request.into_body()).await.unwrap();

                let request = serde_json::from_slice::<types::TunnelRequest>(&full_body).unwrap();

                assert_eq!(request.name, "test");
                assert_eq!(request.tunnel_secret, "secret");

                let mut resp = CloudflareResponse::<types::Tunnel>::default();

                resp.result = Some(Tunnel::default());
                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), types::Tunnel::default());
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_update_tunnel() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10002);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.update_tunnel("tunnel_id", types::TunnelRequest{
                        name: "test".to_string(),
                        tunnel_secret: "secret".to_string(),
                    }).await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::PATCH);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel/tunnel_id");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");
                assert_eq!(request.headers().get("Content-Type").unwrap(), "application/json");

                let full_body = hyper::body::to_bytes(request.into_body()).await.unwrap();

                let request = serde_json::from_slice::<types::TunnelRequest>(&full_body).unwrap();

                assert_eq!(request.name, "test");
                assert_eq!(request.tunnel_secret, "secret");

                let mut resp = CloudflareResponse::<types::Tunnel>::default();

                resp.result = Some(Tunnel::default());
                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), types::Tunnel::default());
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_get_tunnel() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10003);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.get_tunnel("tunnel_id").await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::GET);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel/tunnel_id");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");

                let mut resp = CloudflareResponse::<types::Tunnel>::default();

                resp.result = Some(Tunnel::default());
                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), types::Tunnel::default());
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_delete_tunnel() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10004);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.delete_tunnel("tunnel_id").await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::DELETE);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel/tunnel_id");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");

                let mut resp = CloudflareResponse::<types::Tunnel>::default();

                resp.result = Some(Tunnel::default());
                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), types::Tunnel::default());
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_list_tunnel_connections() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10005);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.list_tunnel_connections("tunnel_id").await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::GET);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel/tunnel_id/connections");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");

                let mut resp = CloudflareResponse::<Vec<TunnelConnection>>::default();

                let result = vec![TunnelConnection::default()];
                resp.result = Some(result.clone());
                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), result);
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_cleanup_tunnel_connections() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiToken("token".to_string()),
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10006);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.clean_up_tunnel_connections("tunnel_id").await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::DELETE);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel/tunnel_id/connections");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("Authorization").unwrap(), "Bearer token");

                let mut resp = CloudflareResponse::<()>::default();

                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), ());
            } => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_get_tunnel_token() {
        let mut client = Client::new(
            "account_id".to_string(),
            Auth::ApiKey {
                key: "some-api-key".to_string(),
                email: "some-email".to_string(),
            },
        );

        let (response_send, mut request_recv, server) = setup_client(&mut client, 10007);

        select! {
            _ = server => panic!("server should not have exited"),
            _ = async move {
                let handle = tokio::spawn(async move {
                    client.get_tunnel_token("some_tunnel_id").await.unwrap()
                });

                let request = request_recv.recv().await.unwrap();

                assert_eq!(request.method(), Method::GET);
                assert_eq!(request.uri().path(), "/accounts/account_id/cfd_tunnel/some_tunnel_id/token");
                assert_eq!(request.uri().query(), None);
                assert_eq!(request.headers().get("X-Auth-Email").unwrap(), "some-email");
                assert_eq!(request.headers().get("X-Auth-Key").unwrap(), "some-api-key");

                let mut resp = CloudflareResponse::<String>::default();
                resp.result = Some("some-token".to_string());
                resp.success = true;

                response_send.send(Response::new(Body::from(serde_json::to_string(&resp).unwrap()))).await.unwrap();

                assert_eq!(handle.await.unwrap(), "some-token");
            } => assert!(true),
        }
    }
}
