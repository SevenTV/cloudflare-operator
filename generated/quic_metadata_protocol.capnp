@0xb29021ef7421cc32;

using Rust = import "rust.capnp";
$Rust.parentModule("capnp::raw");


struct ConnectRequest{
	dest @0 :Text;
	type @1 :ConnectionType;
	metadata @2 :List(Metadata);
}

enum ConnectionType{
	http @0;
	websocket @1;
	tcp @2;
}

struct Metadata {
    key @0 :Text;
	val @1 :Text;
}

struct ConnectResponse{
	error @0 :Text;
	metadata @1 :List(Metadata);
}
