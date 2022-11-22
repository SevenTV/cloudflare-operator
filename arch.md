# Cloudflared Tunnel Run steps

## Inputs

- Tunnel Token

## Steps

### Check if proxydns

- We are only interested in supporting tunnels because that is the only feature we need.

### Check if post-quantum

- post-quantum is a cryptograpic sharing method which aims to be secure vs quantum computers

### DNS lookup to get protocol percentage

- We do a DNS TXT look up on the domain `protocol-v2.argotunnel.com` to return an array JSON structs.

    ```ts
    [
        {
            "protocol": string, // So far this seems to be only http2 and quic
            "percentage": number // this number seems to be only 100, however I am not sure what it represents.
        }
    ]
    ```

### All our tunnels will be named since we are looking to proxy data

### Named Tunnel Steps

- Generate a new random UUID for the client.

- Feature options the defaults are `["allow_remote_config", "serialized_headers", "support_datagram_v2"]`

    ```ts
        {
            "allow_remote_config": "I assume this is the tunnel config from UI which is what we have been using",
            "serialized_headers": "I am not sure what this does however we have been using it",
            "support_datagram_v2": "I am also not sure what this is either but we have been using it",
            "quick_reconnects": "again not sure what this does, not enabled by default",
            "postquantum": "cryptographic security vs quantum computers, not enabled by default enabled"
        }
    ```

- It seems that if we use a remote config we always use QUIC/HTTP2. These values become hardcoded.

- Warprouting wont be supported as it doesnt work with the ingress impl. Mainly used for private networks.

- Their implmentation is weird, for us we will pick QUIC if that fails use HTTP2

- They create a map of TLS configs for edge (cloudflare) servers. (they do it for all supported protocols, however we can just do it for quic and http2)

- We pick the Edge IP version (ipv4 or ipv6)

- Find the local instance ipv4 (and or ipv6) address

- They seem to create an ICMP proxy not sure why yet. (seems optional)

- They then setup logic for routing requests (the ingress part)

- Then they create the tunnel daemon (we need this part)

- We then look for region servers

    ```ts
    // This is done by doing a DNS lookup on the following
    var region = "" // when region is not supplied we use the global servers.
    const srvService = "v2-origintunneld"
    const srvProto = "tcp"
    const srvName = "argotunnel.com"

    let regionalSrvService = srvService;
    if region != "" {
        regionalSrvService = region + "-" + srvService
    }

    dns_lookup("srv", "_"+regionalSrvService+"._"+srvProto+"."+srvName)
    ```

- There is some authentication logic, will have to go over this during implementation too boring to look at now.

- They also have some error handling depending on the ip version, not sure what that is about.

- We then start the daemon.

- If we have ICMP we start that here too.

- We start the first tunnel before starting other tunnels.

- We then handle requets from quic/http2 streams.

- They use capnproto to generate the code used protocol buffers.

    For config autoupdate, and control data from the edge servers.
