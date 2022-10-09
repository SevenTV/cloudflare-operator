use anyhow::Result;

pub fn main() -> Result<()> {
    capnpc::CompilerCommand::new()
        .file(format!("tunnelrpc.capnp"))
        .run()?;

    Ok(())
}
