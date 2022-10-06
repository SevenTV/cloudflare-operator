#! /bin/sh
# rebuild if necessary
make
# run with some arguments
./target/release/cloudflared-ingress "$@"
