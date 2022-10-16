use anyhow::{anyhow, Result};

pub fn main() -> Result<()> {
    capnpc::CompilerCommand::new()
        .file("tunnelrpc.capnp")
        .file("quic_metadata_protocol.capnp")
        .run()
        .map_err(|e| anyhow!("failed to build {}", e))?;

    Ok(())
}
