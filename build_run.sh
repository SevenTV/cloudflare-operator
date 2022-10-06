#! /bin/sh

set -e

# rebuild if necessary
make build
# run with some arguments
./target/release/cloudflared-ingress "$@"
