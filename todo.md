# todo

## We need to

- fetch k8s ingress resources.
- secure a resource lease to ensure we are the only controller managing it.
- create a cloudflared tunnel to proxy the ingress resources.
        - this will require us to implement cloudflared tunnels in rust.
        - we will need to implement a cloudflared tunnel client.
        - we will have to use quic and or http2 to proxy the ingress resources.
- update the ingress resource with the cloudflared tunnel URL.
- watch for changes to the ingress resource and update the proxy rules.
- watch for changes to the endpointslices and update the proxy rules, removing or adding ip targets as needed.
- watch for changes to secrets and update the proxy rules with new credentials if TLS is enabled.

## Features

- upsteam tls
- upstream http2
- upstream websocket
- upstream grpc
- upstream eventstream
- upstream keepalive
- healthchecks are implemented by the pod liveness/readiness probe
- ingress class
- ingress without class
- ingress with multiple hosts
- ingress with multiple paths
- default ingress backend
- ingress with multiple backends

## Things we will not support

- edge tls certificates (we cannot support this because this is managed by cloudflare)

## Optionals

- CRDs for ingress resources, since these are very complex and its not apart of the k8s spec and annotations are a mess.
- CRDs for cloudflared tunnels, so we can manage them outside of the ingress resource.
        (We would likely have to make a separate controller for this, a controller to manage this controller)
- manage cloudflare DNS records for the ingress resources.
- manage cloudflare firewall rules for the ingress resources.            (maybe?)
- manage cloudflare load balancer pools for the ingress resources.       (maybe?)
- manage cloudflare load balancer monitors for the ingress resources.    (maybe?)
- manage cloudflare load balancer pools for the ingress resources.       (maybe?)

## useful links

- <https://github.com/kube-rs/kube> k8s-api

- <https://docs.rs/pipe/latest/pipe/index.html> memory pipe

- <https://tokio.rs/tokio> async runtime

- <https://github.com/crossbeam-rs/crossbeam> thread tools

- <https://github.com/rayon-rs/rayon> thread tools

- <https://github.com/hyperium/hyper> async http client

- <https://github.com/quinn-rs/quinn> async quic impl

- <https://github.com/aws/s2n-quic> async quic impl

- <https://github.com/hyperium/h3> async http3 protocol using quinn or s2n-quic

- <https://github.com/cloudflare/quiche> sync http3 protocol (designed to be used with C)

- <https://github.com/briansmith/ring> cryptography library
