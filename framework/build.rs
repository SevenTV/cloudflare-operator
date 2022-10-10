use anyhow::Result;

pub fn main() -> Result<()> {
    capnpc::CompilerCommand::new()
        .file("tunnelrpc.capnp")
        .file("quic_metadata_protocol.capnp")
        .run()?;

    Ok(())
}
